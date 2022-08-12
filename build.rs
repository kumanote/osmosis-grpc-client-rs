use std::env;
use std::path::PathBuf;
use tonic_build;

fn main() {
    let output_dir = match env::var("OUT_DIR") {
        Ok(s) => PathBuf::from(s),
        Err(_) => {
            let d = env::current_dir().unwrap();
            d.join("src/prost")
        }
    };
    let is_empty = output_dir
        .read_dir()
        .expect("directory must exist")
        .next()
        .is_none();
    if is_empty {
        tonic_build::configure()
            .build_client(true)
            .build_server(false)
            .out_dir("src/prost")
            .compile(
                &[
                    "src/proto/cosmos/crypto/secp256r1/keys.proto",
                    "src/proto/cosmos/crypto/multisig/v1beta1/multisig.proto",
                    "src/proto/cosmos/crypto/multisig/keys.proto",
                    "src/proto/cosmos/crypto/secp256k1/keys.proto",
                    "src/proto/cosmos/crypto/ed25519/keys.proto",
                    "src/proto/cosmos/upgrade/v1beta1/upgrade.proto",
                    "src/proto/cosmos/upgrade/v1beta1/query.proto",
                    "src/proto/cosmos/feegrant/v1beta1/tx.proto",
                    "src/proto/cosmos/feegrant/v1beta1/query.proto",
                    "src/proto/cosmos/feegrant/v1beta1/genesis.proto",
                    "src/proto/cosmos/feegrant/v1beta1/feegrant.proto",
                    "src/proto/cosmos/mint/v1beta1/query.proto",
                    "src/proto/cosmos/mint/v1beta1/genesis.proto",
                    "src/proto/cosmos/mint/v1beta1/mint.proto",
                    "src/proto/cosmos/evidence/v1beta1/tx.proto",
                    "src/proto/cosmos/evidence/v1beta1/evidence.proto",
                    "src/proto/cosmos/evidence/v1beta1/query.proto",
                    "src/proto/cosmos/evidence/v1beta1/genesis.proto",
                    "src/proto/cosmos/auth/v1beta1/query.proto",
                    "src/proto/cosmos/auth/v1beta1/genesis.proto",
                    "src/proto/cosmos/auth/v1beta1/auth.proto",
                    "src/proto/cosmos/bank/v1beta1/tx.proto",
                    "src/proto/cosmos/bank/v1beta1/bank.proto",
                    "src/proto/cosmos/bank/v1beta1/query.proto",
                    "src/proto/cosmos/bank/v1beta1/authz.proto",
                    "src/proto/cosmos/bank/v1beta1/genesis.proto",
                    "src/proto/cosmos/capability/v1beta1/capability.proto",
                    "src/proto/cosmos/capability/v1beta1/genesis.proto",
                    "src/proto/cosmos/distribution/v1beta1/tx.proto",
                    "src/proto/cosmos/distribution/v1beta1/distribution.proto",
                    "src/proto/cosmos/distribution/v1beta1/query.proto",
                    "src/proto/cosmos/distribution/v1beta1/genesis.proto",
                    "src/proto/cosmos/crisis/v1beta1/tx.proto",
                    "src/proto/cosmos/crisis/v1beta1/genesis.proto",
                    "src/proto/cosmos/tx/signing/v1beta1/signing.proto",
                    "src/proto/cosmos/tx/v1beta1/tx.proto",
                    "src/proto/cosmos/tx/v1beta1/service.proto",
                    "src/proto/cosmos/vesting/v1beta1/tx.proto",
                    "src/proto/cosmos/vesting/v1beta1/vesting.proto",
                    "src/proto/cosmos/staking/v1beta1/tx.proto",
                    "src/proto/cosmos/staking/v1beta1/query.proto",
                    // "src/proto/cosmos/staking/v1beta1/authz.proto",
                    "src/proto/cosmos/staking/v1beta1/genesis.proto",
                    "src/proto/cosmos/staking/v1beta1/staking.proto",
                    "src/proto/cosmos/genutil/v1beta1/genesis.proto",
                    "src/proto/cosmos/params/v1beta1/query.proto",
                    "src/proto/cosmos/params/v1beta1/params.proto",
                    "src/proto/cosmos/authz/v1beta1/tx.proto",
                    "src/proto/cosmos/authz/v1beta1/query.proto",
                    "src/proto/cosmos/authz/v1beta1/event.proto",
                    "src/proto/cosmos/authz/v1beta1/authz.proto",
                    "src/proto/cosmos/authz/v1beta1/genesis.proto",
                    "src/proto/cosmos/slashing/v1beta1/tx.proto",
                    "src/proto/cosmos/slashing/v1beta1/slashing.proto",
                    "src/proto/cosmos/slashing/v1beta1/query.proto",
                    "src/proto/cosmos/slashing/v1beta1/genesis.proto",
                    "src/proto/cosmos/base/abci/v1beta1/abci.proto",
                    "src/proto/cosmos/base/kv/v1beta1/kv.proto",
                    "src/proto/cosmos/base/snapshots/v1beta1/snapshot.proto",
                    "src/proto/cosmos/base/v1beta1/coin.proto",
                    "src/proto/cosmos/base/tendermint/v1beta1/query.proto",
                    "src/proto/cosmos/base/query/v1beta1/pagination.proto",
                    "src/proto/cosmos/base/store/v1beta1/commit_info.proto",
                    "src/proto/cosmos/base/store/v1beta1/listening.proto",
                    "src/proto/cosmos/base/reflection/v1beta1/reflection.proto",
                    "src/proto/cosmos/base/reflection/v2alpha1/reflection.proto",
                    "src/proto/cosmos/gov/v1beta1/tx.proto",
                    "src/proto/cosmos/gov/v1beta1/gov.proto",
                    "src/proto/cosmos/gov/v1beta1/query.proto",
                    "src/proto/cosmos/gov/v1beta1/genesis.proto",
                    "src/proto/ibc/core/types/v1/genesis.proto",
                    "src/proto/ibc/core/connection/v1/tx.proto",
                    "src/proto/ibc/core/connection/v1/query.proto",
                    "src/proto/ibc/core/connection/v1/connection.proto",
                    "src/proto/ibc/core/connection/v1/genesis.proto",
                    "src/proto/ibc/core/commitment/v1/commitment.proto",
                    "src/proto/ibc/core/channel/v1/tx.proto",
                    "src/proto/ibc/core/channel/v1/query.proto",
                    "src/proto/ibc/core/channel/v1/genesis.proto",
                    "src/proto/ibc/core/channel/v1/channel.proto",
                    "src/proto/ibc/core/client/v1/tx.proto",
                    "src/proto/ibc/core/client/v1/client.proto",
                    "src/proto/ibc/core/client/v1/query.proto",
                    "src/proto/ibc/core/client/v1/genesis.proto",
                    "src/proto/ibc/lightclients/solomachine/v1/solomachine.proto",
                    "src/proto/ibc/lightclients/solomachine/v2/solomachine.proto",
                    "src/proto/ibc/lightclients/tendermint/v1/tendermint.proto",
                    "src/proto/ibc/lightclients/localhost/v1/localhost.proto",
                    "src/proto/ibc/applications/transfer/v1/tx.proto",
                    "src/proto/ibc/applications/transfer/v1/transfer.proto",
                    "src/proto/ibc/applications/transfer/v1/query.proto",
                    "src/proto/ibc/applications/transfer/v1/genesis.proto",
                    "src/proto/ibc/applications/transfer/v2/packet.proto",
                    "src/proto/ibc/applications/interchain_accounts/v1/account.proto",
                    "src/proto/ibc/applications/interchain_accounts/v1/packet.proto",
                    "src/proto/ibc/applications/interchain_accounts/v1/metadata.proto",
                    "src/proto/ibc/applications/interchain_accounts/v1/genesis.proto",
                    "src/proto/ibc/applications/interchain_accounts/host/v1/host.proto",
                    "src/proto/ibc/applications/interchain_accounts/host/v1/query.proto",
                    "src/proto/ibc/applications/interchain_accounts/controller/v1/query.proto",
                    "src/proto/ibc/applications/interchain_accounts/controller/v1/controller.proto",
                ],
                &["src/proto/", "src/proto/confio/"],
            )
            .unwrap();
    }
}
