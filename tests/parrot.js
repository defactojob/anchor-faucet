const anchor = require('@project-serum/anchor');
const srm = require('@project-serum/common');
const TokenInstructions = require("@project-serum/serum").TokenInstructions;

const { PublicKey } = anchor.web3

const { inspect } = require("util");
PublicKey.prototype[inspect.custom] = function () {
  return {
    type: "PublicKey",
    base58: this.toBase58(),
    hex: this.toBuffer().toString("hex"),
  };
};

describe('parrot', () => {
  // srm.createMint()

  // new
  // anchor.web3.PublicKey.createProgramAddress()

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider);


  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Parrot;

    const faucet = new anchor.web3.Account();

    const [mintAuth, nonce] = await anchor.web3.PublicKey.findProgramAddress([
      faucet.publicKey.toBuffer()
    ], program.programId)

    const mint = await srm.createMint(provider, mintAuth, 8)

    console.log("faucet", faucet.publicKey.toString())
    console.log("mint", mint.toString())
    console.log("mint", mint)
    console.log("mintAuth", mintAuth.toString())

    const tx = await program.rpc.initialize(nonce, {
      accounts: {
        faucet: faucet.publicKey,
        mint,
        tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      instructions: [
        await program.account.faucet.createInstruction(faucet),
      ],
      signers: [faucet]
    });

    // const receiver = new anchor.web3.Account()
    const receiver = await srm.createTokenAccount(provider, mint, provider.wallet.publicKey)
    await program.rpc.drip({
      accounts: {
        faucet: faucet.publicKey,
        mint,
        mintAuth,
        receiver,
        tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
      }
    })


    const faucetAccount = await program.account.faucet(faucet.publicKey);
    console.log(faucetAccount)

    console.log("receiver", await srm.getTokenAccount(provider, receiver))

    // console.log("Your transaction signature", tx);
  });
});
