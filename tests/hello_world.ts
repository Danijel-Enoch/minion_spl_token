import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MinionToken } from "../target/types/MinionToken";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("hello_world", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const minion_program = anchor.workspace.MinionToken as Program<MinionToken>;

	it("Is initialized!", async () => {
		// Add your test here.
		const tx = await minion_program.methods.initToken({
			name: "",
			symbol: "",
			uri: "",
			decimals: 0,
		});
		console.log("Your transaction signature", tx);
	});
});
