const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("TrustLayerReputation", function () {
  let reputation;
  let owner, escrow, disputeContract, guide1, other;

  beforeEach(async function () {
    [owner, escrow, disputeContract, guide1, other] = await ethers.getSigners();
    const Reputation = await ethers.getContractFactory("TrustLayerReputation");
    reputation = await Reputation.deploy();
    await reputation.setEscrowContract(escrow.address);
    await reputation.setDisputeContract(disputeContract.address);
  });

  describe("recordCompletion", function () {
    it("should increase score by 5 when called by escrow", async function () {
      await reputation.connect(escrow).recordCompletion(guide1.address);
      expect(await reputation.getScore(guide1.address)).to.equal(5);
    });

    it("should track completed bookings count", async function () {
      await reputation.connect(escrow).recordCompletion(guide1.address);
      await reputation.connect(escrow).recordCompletion(guide1.address);
      const record = await reputation.getReputation(guide1.address);
      expect(record.completedBookings).to.equal(2);
      expect(record.score).to.equal(10);
    });

    it("should revert if not called by escrow", async function () {
      await expect(reputation.connect(other).recordCompletion(guide1.address))
        .to.be.revertedWith("Only escrow");
    });
  });

  describe("recordDispute", function () {
    it("should decrease score when called by dispute contract", async function () {
      await reputation.connect(escrow).recordCompletion(guide1.address);
      await reputation.connect(escrow).recordCompletion(guide1.address);
      await reputation.connect(escrow).recordCompletion(guide1.address); // score = 15
      await reputation.connect(disputeContract).recordDispute(guide1.address); // -10 = 5
      expect(await reputation.getScore(guide1.address)).to.equal(5);
    });

    it("should not go below 0", async function () {
      await reputation.connect(disputeContract).recordDispute(guide1.address);
      expect(await reputation.getScore(guide1.address)).to.equal(0);
    });

    it("should revert if not called by dispute contract", async function () {
      await expect(reputation.connect(other).recordDispute(guide1.address))
        .to.be.revertedWith("Only dispute contract");
    });
  });

  describe("access control", function () {
    it("should revert setEscrowContract if not owner", async function () {
      await expect(reputation.connect(other).setEscrowContract(escrow.address))
        .to.be.reverted;
    });
  });
});
