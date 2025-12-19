use std::collections::HashMap;

use axion_miner::miner::LatestBlockDetails;

use crate::services::errors_frontend::AxionFrontendError;

pub struct OnStart;

impl OnStart {
    pub fn get_block_height_and_hash(
        block_headers: Vec<LatestBlockDetails>,
    ) -> Result<LatestBlockDetails, AxionFrontendError> {
        let mut highest_block_height = 0;
        let mut temp_vec = vec![];
        for i in block_headers {
            if i.block_height > highest_block_height {
                highest_block_height = i.block_height;
                temp_vec.clear();
                temp_vec.push(i);
            } else if i.block_height == highest_block_height {
                temp_vec.push(i);
            }
        }

        if temp_vec.len() == 1 {
            return Ok(temp_vec.remove(0));
        }

        let mut counts = HashMap::new();
        for i in &temp_vec {
            counts
                .entry(format!("{},{}", i.block_hash, i.block_height))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let mut highest = 0;
        let mut string = String::new();

        for (key, count) in counts {
            if count > highest {
                highest = count;
                string = key;
            }
        }

        let (block_hash, block_height) = string.split_once(",").unwrap();

        let latest_block = LatestBlockDetails {
            block_hash: block_hash.to_owned(),
            block_height: block_height.to_owned().parse().unwrap(),
        };
        Ok(latest_block)
    }
}
