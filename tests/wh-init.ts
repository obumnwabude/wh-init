import {
  AnchorProvider,
  Program,
  getProvider,
  setProvider,
  web3,
  workspace,
} from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { getContracts } from "@wormhole-foundation/sdk";
import { WhInit } from "../target/types/wh_init";

describe("wh-init", () => {
  // Configure the client to use the local cluster.
  setProvider(AnchorProvider.env());
  const program = workspace.WhInit as Program<WhInit>;

  it("Is initialized with wormhole", async () => {
    const { coreBridge, tokenBridge } = getContracts("Devnet", "Solana");
    if (!coreBridge) throw "Got Invalid coreBridge contract from Wormhole";
    if (!tokenBridge) throw "Got Invalid tokenBridge contract from Wormhole";

    const wormholeProgram = new PublicKey(coreBridge);
    const tokenBridgeProgram = new PublicKey(tokenBridge);
    const config = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    )[0];
    const wormholeBridge = PublicKey.findProgramAddressSync(
      [Buffer.from("Bridge")],
      wormholeProgram
    )[0];
    const tokenBridgeConfig = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      tokenBridgeProgram
    )[0];
    const owner = getProvider().publicKey;
    const systemProgram = new PublicKey(web3.SystemProgram.programId);

    const accounts = {
      owner,
      systemProgram,
      config,
      wormholeProgram,
      tokenBridgeProgram,
      wormholeBridge,
      tokenBridgeConfig,
    };

    const tx = await program.methods.initialize().accounts(accounts).rpc();
    console.log("Your transaction signature", tx);
  });
});
