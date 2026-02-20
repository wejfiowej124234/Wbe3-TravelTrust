// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/Ownable.sol";

interface IStaking {
    function isQualified(address guide) external view returns (bool);
}

interface IReputation {
    function recordCompletion(address guide) external;
}

contract TrustLayerEscrow is Ownable {
    enum BookingStatus { PENDING, CONFIRMED, COMPLETED, CANCELLED, DISPUTED }

    struct Booking {
        uint256 id;
        address traveler;
        address guide;
        uint256 amount;
        BookingStatus status;
        uint256 createdAt;
        uint256 completedAt;
        string serviceDescription;
    }

    IStaking public stakingContract;
    IReputation public reputationContract;
    address public disputeContract;

    uint256 private nextBookingId;
    mapping(uint256 => Booking) public bookings;
    mapping(address => uint256[]) public travelerBookings;
    mapping(address => uint256[]) public guideBookings;

    event BookingCreated(uint256 indexed bookingId, address indexed traveler, address indexed guide, uint256 amount);
    event BookingConfirmed(uint256 indexed bookingId);
    event BookingCompleted(uint256 indexed bookingId);
    event BookingCancelled(uint256 indexed bookingId);
    event BookingDisputed(uint256 indexed bookingId);

    constructor(address _staking) Ownable(msg.sender) {
        require(_staking != address(0), "Invalid staking address");
        stakingContract = IStaking(_staking);
    }

    modifier onlyDisputeContract() {
        require(msg.sender == disputeContract, "Only dispute contract");
        _;
    }

    function setReputationContract(address _reputation) external onlyOwner {
        require(_reputation != address(0), "Invalid address");
        reputationContract = IReputation(_reputation);
    }

    function setDisputeContract(address _dispute) external onlyOwner {
        require(_dispute != address(0), "Invalid address");
        disputeContract = _dispute;
    }

    function createBooking(address guide, string calldata serviceDescription) external payable returns (uint256) {
        require(msg.value > 0, "Payment required");
        require(stakingContract.isQualified(guide), "Guide not qualified");
        require(guide != msg.sender, "Cannot book yourself");

        uint256 bookingId = nextBookingId++;
        bookings[bookingId] = Booking({
            id: bookingId,
            traveler: msg.sender,
            guide: guide,
            amount: msg.value,
            status: BookingStatus.PENDING,
            createdAt: block.timestamp,
            completedAt: 0,
            serviceDescription: serviceDescription
        });

        travelerBookings[msg.sender].push(bookingId);
        guideBookings[guide].push(bookingId);

        emit BookingCreated(bookingId, msg.sender, guide, msg.value);
        return bookingId;
    }

    function confirmBooking(uint256 bookingId) external {
        Booking storage booking = bookings[bookingId];
        require(booking.guide == msg.sender, "Not the guide");
        require(booking.status == BookingStatus.PENDING, "Not pending");

        booking.status = BookingStatus.CONFIRMED;
        emit BookingConfirmed(bookingId);
    }

    function completeBooking(uint256 bookingId) external {
        Booking storage booking = bookings[bookingId];
        require(booking.traveler == msg.sender, "Not the traveler");
        require(booking.status == BookingStatus.CONFIRMED, "Not confirmed");

        booking.status = BookingStatus.COMPLETED;
        booking.completedAt = block.timestamp;

        if (address(reputationContract) != address(0)) {
            reputationContract.recordCompletion(booking.guide);
        }

        (bool success, ) = payable(booking.guide).call{value: booking.amount}("");
        require(success, "Transfer failed");

        emit BookingCompleted(bookingId);
    }

    function cancelBooking(uint256 bookingId) external {
        Booking storage booking = bookings[bookingId];
        require(
            booking.traveler == msg.sender || booking.guide == msg.sender,
            "Not authorized"
        );
        require(booking.status == BookingStatus.PENDING, "Can only cancel pending");

        booking.status = BookingStatus.CANCELLED;

        (bool success, ) = payable(booking.traveler).call{value: booking.amount}("");
        require(success, "Refund failed");

        emit BookingCancelled(bookingId);
    }

    function markDisputed(uint256 bookingId) external onlyDisputeContract {
        Booking storage booking = bookings[bookingId];
        require(
            booking.status == BookingStatus.CONFIRMED || booking.status == BookingStatus.PENDING,
            "Cannot dispute"
        );
        booking.status = BookingStatus.DISPUTED;
        emit BookingDisputed(bookingId);
    }

    /// @notice Releases escrowed funds to the guide; called by dispute contract when resolved in guide's favor.
    function releaseToGuide(uint256 bookingId) external onlyDisputeContract {
        Booking storage booking = bookings[bookingId];
        require(booking.status == BookingStatus.DISPUTED, "Not disputed");
        booking.status = BookingStatus.COMPLETED;
        booking.completedAt = block.timestamp;
        (bool success, ) = payable(booking.guide).call{value: booking.amount}("");
        require(success, "Transfer failed");
    }

    /// @notice Refunds escrowed funds to the traveler; called by dispute contract when resolved in traveler's favor.
    function refundToTraveler(uint256 bookingId) external onlyDisputeContract {
        Booking storage booking = bookings[bookingId];
        require(booking.status == BookingStatus.DISPUTED, "Not disputed");
        booking.status = BookingStatus.CANCELLED;
        (bool success, ) = payable(booking.traveler).call{value: booking.amount}("");
        require(success, "Refund failed");
    }

    function getBooking(uint256 bookingId) external view returns (Booking memory) {
        return bookings[bookingId];
    }

    function getTravelerBookings(address traveler) external view returns (uint256[] memory) {
        return travelerBookings[traveler];
    }

    function getGuideBookings(address guide) external view returns (uint256[] memory) {
        return guideBookings[guide];
    }
}
