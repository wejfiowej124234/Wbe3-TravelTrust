// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/Ownable.sol";

contract TrustLayerStaking is Ownable {
    uint256 public constant MINIMUM_STAKE = 0.1 ether;
    uint256 public constant LOCKUP_PERIOD = 7 days;

    struct Guide {
        uint256 stakeAmount;
        bool isRegistered;
        uint256 registrationTimestamp;
        uint256 unregisteredAt;
    }

    mapping(address => Guide) public guides;

    event GuideRegistered(address indexed guide, uint256 stakeAmount);
    event GuideUnregistered(address indexed guide);
    event StakeWithdrawn(address indexed guide, uint256 amount);

    constructor() Ownable(msg.sender) {}

    function registerGuide() external payable {
        require(!guides[msg.sender].isRegistered, "Already registered");
        require(msg.value >= MINIMUM_STAKE, "Insufficient stake");

        guides[msg.sender] = Guide({
            stakeAmount: msg.value,
            isRegistered: true,
            registrationTimestamp: block.timestamp,
            unregisteredAt: 0
        });

        emit GuideRegistered(msg.sender, msg.value);
    }

    function unregisterGuide() external {
        Guide storage guide = guides[msg.sender];
        require(guide.isRegistered, "Not registered");

        guide.isRegistered = false;
        guide.unregisteredAt = block.timestamp;

        emit GuideUnregistered(msg.sender);
    }

    function withdrawStake() external {
        Guide storage guide = guides[msg.sender];
        require(!guide.isRegistered, "Still registered");
        require(guide.stakeAmount > 0, "No stake to withdraw");
        require(
            block.timestamp >= guide.unregisteredAt + LOCKUP_PERIOD,
            "Lockup period not over"
        );

        uint256 amount = guide.stakeAmount;
        guide.stakeAmount = 0;

        (bool success, ) = payable(msg.sender).call{value: amount}("");
        require(success, "Transfer failed");

        emit StakeWithdrawn(msg.sender, amount);
    }

    function isQualified(address guide) external view returns (bool) {
        Guide memory g = guides[guide];
        return g.isRegistered && g.stakeAmount >= MINIMUM_STAKE;
    }

    function getGuideInfo(address guide) external view returns (Guide memory) {
        return guides[guide];
    }
}
