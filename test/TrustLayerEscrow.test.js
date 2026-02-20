const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("TrustLayerEscrow", function () {
  let staking, escrow, reputation;
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

    await escrow.setReputationContract(await reputation.getAddress());
    await reputation.setEscrowContract(await escrow.getAddress());

    // Register guide
    await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
  });

  describe("createBooking", function () {
    it("should create a booking with qualified guide", async function () {
      await expect(
        escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT })
      )
        .to.emit(escrow, "BookingCreated")
        .withArgs(0, traveler.address, guide1.address, BOOKING_AMOUNT);

      const booking = await escrow.getBooking(0);
      expect(booking.traveler).to.equal(traveler.address);
      expect(booking.guide).to.equal(guide1.address);
      expect(booking.amount).to.equal(BOOKING_AMOUNT);
      expect(booking.status).to.equal(0); // PENDING
    });

    it("should revert if guide is not qualified", async function () {
      await expect(
        escrow.connect(traveler).createBooking(other.address, "Tour", { value: BOOKING_AMOUNT })
      ).to.be.revertedWith("Guide not qualified");
    });

    it("should revert if payment is 0", async function () {
      await expect(
        escrow.connect(traveler).createBooking(guide1.address, "Tour", { value: 0 })
      ).to.be.revertedWith("Payment required");
    });

    it("should revert if booking yourself", async function () {
      await staking.connect(traveler).registerGuide({ value: MINIMUM_STAKE });
      await expect(
        escrow.connect(traveler).createBooking(traveler.address, "Tour", { value: BOOKING_AMOUNT })
      ).to.be.revertedWith("Cannot book yourself");
    });
  });

  describe("confirmBooking", function () {
    it("should allow guide to confirm booking", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });
      await expect(escrow.connect(guide1).confirmBooking(0))
        .to.emit(escrow, "BookingConfirmed")
        .withArgs(0);

      const booking = await escrow.getBooking(0);
      expect(booking.status).to.equal(1); // CONFIRMED
    });

    it("should revert if not the guide", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });
      await expect(escrow.connect(other).confirmBooking(0)).to.be.revertedWith("Not the guide");
    });
  });

  describe("completeBooking", function () {
    it("should complete booking and release payment to guide", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });
      await escrow.connect(guide1).confirmBooking(0);

      const guideBefore = await ethers.provider.getBalance(guide1.address);
      await expect(escrow.connect(traveler).completeBooking(0))
        .to.emit(escrow, "BookingCompleted")
        .withArgs(0);

      const guideAfter = await ethers.provider.getBalance(guide1.address);
      expect(guideAfter - guideBefore).to.equal(BOOKING_AMOUNT);

      const booking = await escrow.getBooking(0);
      expect(booking.status).to.equal(2); // COMPLETED
    });

    it("should update reputation on completion", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });
      await escrow.connect(guide1).confirmBooking(0);
      await escrow.connect(traveler).completeBooking(0);

      const score = await reputation.getScore(guide1.address);
      expect(score).to.equal(5);
    });

    it("should revert if not the traveler", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });
      await escrow.connect(guide1).confirmBooking(0);
      await expect(escrow.connect(other).completeBooking(0)).to.be.revertedWith("Not the traveler");
    });
  });

  describe("cancelBooking", function () {
    it("should cancel pending booking and refund traveler", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });

      const travelerBefore = await ethers.provider.getBalance(traveler.address);
      const tx = await escrow.connect(traveler).cancelBooking(0);
      const receipt = await tx.wait();
      const gasUsed = receipt.gasUsed * receipt.gasPrice;
      const travelerAfter = await ethers.provider.getBalance(traveler.address);

      expect(travelerAfter).to.be.closeTo(travelerBefore + BOOKING_AMOUNT - gasUsed, ethers.parseEther("0.001"));
    });

    it("should revert if booking is confirmed", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "City tour", { value: BOOKING_AMOUNT });
      await escrow.connect(guide1).confirmBooking(0);
      await expect(escrow.connect(traveler).cancelBooking(0)).to.be.revertedWith("Can only cancel pending");
    });
  });

  describe("getters", function () {
    it("should return traveler bookings", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "Tour 1", { value: BOOKING_AMOUNT });
      await escrow.connect(traveler).createBooking(guide1.address, "Tour 2", { value: BOOKING_AMOUNT });
      const ids = await escrow.getTravelerBookings(traveler.address);
      expect(ids.length).to.equal(2);
    });

    it("should return guide bookings", async function () {
      await escrow.connect(traveler).createBooking(guide1.address, "Tour 1", { value: BOOKING_AMOUNT });
      const ids = await escrow.getGuideBookings(guide1.address);
      expect(ids.length).to.equal(1);
    });
  });
});
