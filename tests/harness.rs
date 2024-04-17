use fuels::prelude::*;
use fuels::tx::ContractIdExt;
use fuels::types::Bits256;
use fuels::types::Bytes32;
use fuels::types::ContractId;
abigen!(
    Contract(name = "Pig", abi = "./token/out/debug/pig-abi.json"),
    Predicate(
        name = "Unspendable",
        abi = "./unspendable/out/debug/unspendable-abi.json",
    ),
    Predicate(
        name = "Spendable",
        abi = "./spendable/out/debug/spendable-abi.json",
    ),
);
#[tokio::test]
async fn minting() {
    let mut wallet = launch_provider_and_get_wallet().await.unwrap();
    let mut wallet_2 = WalletUnlocked::new_random(None);
    let contract_id =
        Contract::load_from("./token/out/debug/pig.bin", LoadConfiguration::default())
            .unwrap()
            .deploy(&wallet, TxPolicies::default())
            .await
            .unwrap();
    println!("{contract_id}");
    let pig = Pig::new(contract_id.clone(), wallet.clone());
    let response = pig
        .methods()
        .mint(contract_id.into(), Bits256::zeroed(), 1)
        .call()
        .await
        .unwrap();
    dbg!(response);
    let sub_id = &Bytes32::zeroed();
    let asset_id: AssetId = ContractId::from(pig.contract_id()).asset_id(sub_id).into();
    let configurables = UnspendableConfigurables::default()
        .with_ASSET_ID(asset_id)
        .unwrap();
    let unspendable = Predicate::load_from("./unspendable/out/debug/unspendable.bin")
        .unwrap()
        .with_configurables(configurables);
    let configurables = SpendableConfigurables::default()
        .with_ASSET_ID(asset_id)
        .unwrap()
        .with_OWNER(wallet.address().into())
        .unwrap();
    let spendable = Predicate::load_from("./spendable/out/debug/spendable.bin")
        .unwrap()
        .with_configurables(configurables);
    let (coins, _) = setup_multiple_assets_coins(spendable.address(), 3, 1, 1000);
}
