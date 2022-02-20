import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { RiskEnforcer } from '../target/types/risk_enforcer';

describe('risk_enforcer', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.RiskEnforcer as Program<RiskEnforcer>;

  it('Is initialized!', async () => {
    // market_index = 2 (BTC)
    // sym = test
    const tx = await program.methods.initialize(2, "test").accounts({
      authority: program.provider.wallet.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
