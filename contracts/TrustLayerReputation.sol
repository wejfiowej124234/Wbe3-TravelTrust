// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/Ownable.sol";

contract TrustLayerReputation is Ownable {
    uint256 public constant COMPLETION_POINTS = 5;
    uint256 public constant DISPUTE_PENALTY = 10;

    struct ReputationRecord {
        uint256 score;
        uint256 completedBookings;
        uint256 disputedBookings;
        uint256 lastUpdated;
    }

    mapping(address => ReputationRecord) public reputations;

    address public escrowContract;
    address public disputeContract;

    event ReputationUpdated(address indexed guide, uint256 newScore);
    event EscrowContractSet(address indexed escrowContract);
    event DisputeContractSet(address indexed disputeContract);

    constructor() Ownable(msg.sender) {}

    modifier onlyAuthorized() {
        require(
            msg.sender == escrowContract || msg.sender == disputeContract,
            "Not authorized"
        );
        _;
    }

    function setEscrowContract(address _escrow) external onlyOwner {
        require(_escrow != address(0), "Invalid address");
        escrowContract = _escrow;
        emit EscrowContractSet(_escrow);
    }

    function setDisputeContract(address _dispute) external onlyOwner {
        require(_dispute != address(0), "Invalid address");
        disputeContract = _dispute;
        emit DisputeContractSet(_dispute);
    }

    function recordCompletion(address guide) external {
        require(msg.sender == escrowContract, "Only escrow");
        ReputationRecord storage record = reputations[guide];
        record.score += COMPLETION_POINTS;
        record.completedBookings += 1;
        record.lastUpdated = block.timestamp;
        emit ReputationUpdated(guide, record.score);
    }

    function recordDispute(address guide) external {
        require(msg.sender == disputeContract, "Only dispute contract");
        ReputationRecord storage record = reputations[guide];
        if (record.score >= DISPUTE_PENALTY) {
            record.score -= DISPUTE_PENALTY;
        } else {
            record.score = 0;
        }
        record.disputedBookings += 1;
        record.lastUpdated = block.timestamp;
        emit ReputationUpdated(guide, record.score);
    }

    function getReputation(address guide) external view returns (ReputationRecord memory) {
        return reputations[guide];
    }

    function getScore(address guide) external view returns (uint256) {
        return reputations[guide].score;
    }
}
