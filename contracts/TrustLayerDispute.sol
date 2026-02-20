// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/Ownable.sol";

interface IEscrow {
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
    function getBooking(uint256 bookingId) external view returns (Booking memory);
    function markDisputed(uint256 bookingId) external;
    function releaseToGuide(uint256 bookingId) external;
    function refundToTraveler(uint256 bookingId) external;
}

interface IReputationDispute {
    function recordDispute(address guide) external;
}

contract TrustLayerDispute is Ownable {
    enum DisputeStatus { OPEN, RESOLVED_FOR_TRAVELER, RESOLVED_FOR_GUIDE, ESCALATED }

    struct Dispute {
        uint256 id;
        uint256 bookingId;
        address raisedBy;
        string reason;
        DisputeStatus status;
        uint256 createdAt;
        uint256 resolvedAt;
        string resolution;
    }

    IEscrow public escrowContract;
    IReputationDispute public reputationContract;

    uint256 private nextDisputeId;
    mapping(uint256 => Dispute) public disputes;
    mapping(uint256 => uint256) public bookingToDispute; // bookingId => disputeId
    mapping(uint256 => bool) public bookingHasDispute;

    event DisputeRaised(uint256 indexed disputeId, uint256 indexed bookingId, address indexed raisedBy);
    event DisputeResolved(uint256 indexed disputeId, DisputeStatus status, string resolution);

    constructor(address _escrow, address _reputation) Ownable(msg.sender) {
        require(_escrow != address(0), "Invalid escrow address");
        require(_reputation != address(0), "Invalid reputation address");
        escrowContract = IEscrow(_escrow);
        reputationContract = IReputationDispute(_reputation);
    }

    function raiseDispute(uint256 bookingId, string calldata reason) external returns (uint256) {
        require(!bookingHasDispute[bookingId], "Dispute already exists");

        IEscrow.Booking memory booking = escrowContract.getBooking(bookingId);
        require(
            booking.traveler == msg.sender || booking.guide == msg.sender,
            "Not authorized"
        );
        require(
            booking.status == IEscrow.BookingStatus.CONFIRMED || booking.status == IEscrow.BookingStatus.PENDING,
            "Cannot dispute this booking"
        );

        escrowContract.markDisputed(bookingId);

        uint256 disputeId = nextDisputeId++;
        disputes[disputeId] = Dispute({
            id: disputeId,
            bookingId: bookingId,
            raisedBy: msg.sender,
            reason: reason,
            status: DisputeStatus.OPEN,
            createdAt: block.timestamp,
            resolvedAt: 0,
            resolution: ""
        });

        bookingToDispute[bookingId] = disputeId;
        bookingHasDispute[bookingId] = true;

        emit DisputeRaised(disputeId, bookingId, msg.sender);
        return disputeId;
    }

    function resolveDispute(uint256 disputeId, bool favorTraveler, string calldata resolution) external onlyOwner {
        Dispute storage dispute = disputes[disputeId];
        require(dispute.status == DisputeStatus.OPEN, "Dispute not open");

        dispute.resolvedAt = block.timestamp;
        dispute.resolution = resolution;

        IEscrow.Booking memory booking = escrowContract.getBooking(dispute.bookingId);

        if (favorTraveler) {
            dispute.status = DisputeStatus.RESOLVED_FOR_TRAVELER;
            escrowContract.refundToTraveler(dispute.bookingId);
            reputationContract.recordDispute(booking.guide);
        } else {
            dispute.status = DisputeStatus.RESOLVED_FOR_GUIDE;
            escrowContract.releaseToGuide(dispute.bookingId);
        }

        emit DisputeResolved(disputeId, dispute.status, resolution);
    }

    function getDispute(uint256 disputeId) external view returns (Dispute memory) {
        return disputes[disputeId];
    }

    function getBookingDispute(uint256 bookingId) external view returns (Dispute memory) {
        require(bookingHasDispute[bookingId], "No dispute for booking");
        return disputes[bookingToDispute[bookingId]];
    }
}
