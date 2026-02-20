const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("TrustLayerDispute", function () {
  let staking, escrow, reputation, dispute;
  let owner, traveler, guide1, other;
  const MINIMUM_STAKE = ethers.parseEther("0.1");
  const BOOKING_AMOUNT = ethers.parseEther("0.5");

  beforeEach(async function () {
    [owner, traveler, guide1, other] = await ethers.getSigners();

    const Staking = await ethers.getContractFactory("TrustLayerStaking");
    staking = await Staking.deploy();

    const Escrow = await ethers.getContractFactory("TrustLayerEscrow");
    escrow = await Escrow.deploy(await staking.getAddress());

    const Reputation = await ethers.getContractFactory("TrustLayerReputation");
    reputation = await Reputation.deploy();

    const Dispute = await ethers.getContractFactory("TrustLayerDispute");
    dispute = await Dispute.deploy(await escrow.getAddress(), await reputation.getAddress());

    await escrow.setReputationContract(await reputation.getAddress());
    await escrow.setDisputeContract(await dispute.getAddress());
    await reputation.setEscrowContract(await escrow.getAddress());
    await reputation.setDisputeContract(await dispute.getAddress());

    // Register guide and create a confirmed booking
    await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
    await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });
    await escrow.connect(guide1).confirmBooking(0);
  });

  describe("raiseDispute", function () {
    it("should allow traveler to raise a dispute", async function () {
      await expect(dispute.connect(traveler).raiseDispute(0, "Guide did not show up"))
        .to.emit(dispute, "DisputeRaised")
        .withArgs(0, 0, traveler.address);

      const d = await dispute.getDispute(0);
      expect(d.status).to.equal(0); // OPEN
      expect(d.bookingId).to.equal(0);
      expect(d.raisedBy).to.equal(traveler.address);
    });

    it("should allow guide to raise a dispute", async function () {
      await expect(dispute.connect(guide1).raiseDispute(0, "Traveler no-show"))
        .to.emit(dispute, "DisputeRaised");
    });

    it("should revert if not traveler or guide", async function () {
      await expect(dispute.connect(other).raiseDispute(0, "reason"))
        .to.be.revertedWith("Not authorized");
    });

    it("should revert if dispute already exists", async function () {
      await dispute.connect(traveler).raiseDispute(0, "reason");
      await expect(dispute.connect(traveler).raiseDispute(0, "reason2"))
        .to.be.revertedWith("Dispute already exists");
    });
  });

  describe("resolveDispute", function () {
    beforeEach(async function () {
      await dispute.connect(traveler).raiseDispute(0, "Guide did not show up");
    });

    it("should resolve in favor of traveler and refund", async function () {
      const travelerBefore = await ethers.provider.getBalance(traveler.address);
      await expect(dispute.connect(owner).resolveDispute(0, true, "Traveler wins"))
        .to.emit(dispute, "DisputeResolved");

      const travelerAfter = await ethers.provider.getBalance(traveler.address);
      expect(travelerAfter - travelerBefore).to.equal(BOOKING_AMOUNT);

      const d = await dispute.getDispute(0);
      expect(d.status).to.equal(1); // RESOLVED_FOR_TRAVELER
    });

    it("should penalize guide reputation when resolved for traveler", async function () {
      const scoreBefore = await reputation.getScore(guide1.address);
      await dispute.connect(owner).resolveDispute(0, true, "Traveler wins");
      const scoreAfter = await reputation.getScore(guide1.address);
      expect(scoreAfter).to.equal(0); // was 0, can't go below 0
    });

    it("should resolve in favor of guide and release funds", async function () {
      const guideBefore = await ethers.provider.getBalance(guide1.address);
      await expect(dispute.connect(owner).resolveDispute(0, false, "Guide wins"))
        .to.emit(dispute, "DisputeResolved");

      const guideAfter = await ethers.provider.getBalance(guide1.address);
      expect(guideAfter - guideBefore).to.equal(BOOKING_AMOUNT);

      const d = await dispute.getDispute(0);
      expect(d.status).to.equal(2); // RESOLVED_FOR_GUIDE
    });

    it("should revert if not owner", async function () {
      await expect(dispute.connect(other).resolveDispute(0, true, "reason"))
        .to.be.reverted;
    });

    it("should revert if dispute not open", async function () {
      await dispute.connect(owner).resolveDispute(0, true, "resolved");
      await expect(dispute.connect(owner).resolveDispute(0, false, "again"))
        .to.be.revertedWith("Dispute not open");
    });
  });

  describe("getBookingDispute", function () {
    it("should return dispute by bookingId", async function () {
      await dispute.connect(traveler).raiseDispute(0, "reason");
      const d = await dispute.getBookingDispute(0);
      expect(d.bookingId).to.equal(0);
    });

    it("should revert if no dispute for booking", async function () {
      await expect(dispute.getBookingDispute(0)).to.be.revertedWith("No dispute for booking");
    });
  });
});
