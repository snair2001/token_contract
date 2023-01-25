use crate::*;

#[near_bindgen]
impl Contract{

	#[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId, //In frontend I'll handle this part,only thing I need from user is metadata
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) {

        /*assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "only owner"
        );*/
        
        let initial_storage_usage = env::storage_usage();

        let mut royalty = HashMap::new();
        
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

            let mut total_amount: u32 = 0;
            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                total_amount+=amount;
                royalty.insert(account, amount);
            }

            assert!(total_amount <= 4000, "Cannot have more than 40% royalty");
        }

        //log!("The initial storage was {}",initial_storage_usage); //Something for me to check lol
        let token = Token {
            owner_id: receiver_id,
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            royalty,
        };

        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        self.token_metadata_by_id.insert(&token_id, &metadata);

        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo: None,
            }]),
        };

        log!("{}",&nft_mint_log.to_string());

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }

    #[payable]
    pub fn nft_burn(&mut self, token_id: TokenId) {
        assert_one_yocto();

        let owner_id = self.tokens_by_id.get(&token_id).unwrap().owner_id;
        assert_eq!(
            owner_id,
            env::predecessor_account_id(),
            "Token owner only"
        );

        self.internal_remove_token_from_owner(&owner_id, &token_id);

        self.tokens_by_id.remove(&token_id);

        self.token_metadata_by_id.remove(&token_id);

    }
}