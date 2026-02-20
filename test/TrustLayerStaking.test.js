const { expect } = require("chai");
const { ethers } = require("hardhat");
const { time } = require("@nomicfoundation/hardhat-network-helpers");

describe("TrustLayerStaking", function () {
  let staking;
  let owner, guide1, guide2;
  const MINIMUM_STAKE = ethers.parseEther("0.1");
  const LOCKUP_PERIOD = 7 * 24 * 60 * 60; // 7 days in seconds

  beforeEach(async function () {
    [owner, guide1, guide2] = await ethers.getSigners();
    const Staking = await ethers.getContractFactory("TrustLayerStaking");
    staking = await Staking.deploy();
  });

  describe("registerGuide", function () {
    it("should register a guide with sufficient stake", async function () {
      await expect(staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE }))
        .to.emit(staking, "GuideRegistered")
        .withArgs(guide1.address, MINIMUM_STAKE);

      const info = await staking.getGuideInfo(guide1.address);
      expect(info.isRegistered).to.be.true;
      expect(info.stakeAmount).to.equal(MINIMUM_STAKE);
    });

    it("should revert if stake is too low", async function () {
      await expect(
        staking.connect(guide1).registerGuide({ value: ethers.parseEther("0.05") })
      ).to.be.revertedWith("Insufficient stake");
    });

    it("should revert if already registered", async function () {
      await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
      await expect(
        staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE })
      ).to.be.revertedWith("Already registered");
    });
  });

  describe("isQualified", function () {
    it("should return true for registered guide with sufficient stake", async function () {
      await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
      expect(await staking.isQualified(guide1.address)).to.be.true;
    });

    it("should return false for unregistered address", async function () {
      expect(await staking.isQualified(guide2.address)).to.be.false;
    });
  });

  describe("unregisterGuide", function () {
    it("should unregister a registered guide", async function () {
      await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
      await expect(staking.connect(guide1).unregisterGuide())
        .to.emit(staking, "GuideUnregistered")
        .withArgs(guide1.address);

      expect(await staking.isQualified(guide1.address)).to.be.false;
    });

    it("should revert if not registered", async function () {
      await expect(staking.connect(guide1).unregisterGuide()).to.be.revertedWith("Not registered");
    });
  });

  describe("withdrawStake", function () {
    it("should allow withdrawal after lockup period", async function () {
      await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
      await staking.connect(guide1).unregisterGuide();

      await time.increase(LOCKUP_PERIOD + 1);

      const balanceBefore = await ethers.provider.getBalance(guide1.address);
      const tx = await staking.connect(guide1).withdrawStake();
      const receipt = await tx.wait();
      const gasUsed = receipt.gasUsed * receipt.gasPrice;
      const balanceAfter = await ethers.provider.getBalance(guide1.address);

      expect(balanceAfter).to.be.closeTo(balanceBefore + MINIMUM_STAKE - gasUsed, ethers.parseEther("0.001"));

      await expect(staking.connect(guide1).withdrawStake()).to.be.revertedWith("No stake to withdraw");
    });

    it("should revert if still registered", async function () {
      await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
      await expect(staking.connect(guide1).withdrawStake()).to.be.revertedWith("Still registered");
    });

    it("should revert if lockup period not over", async function () {
      await staking.connect(guide1).registerGuide({ value: MINIMUM_STAKE });
      await staking.connect(guide1).unregisterGuide();
      await expect(staking.connect(guide1).withdrawStake()).to.be.revertedWith("Lockup period not over");
    });
  });
});
