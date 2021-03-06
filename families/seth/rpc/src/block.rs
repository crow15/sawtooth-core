/*
 * Copyright 2017 Intel Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

use jsonrpc_core::{Params, Value, Error};
use protobuf;
use uuid;

use error;

use sawtooth_sdk::messaging::stream::*;

use sawtooth_sdk::messages::client::{
    ClientBlockListRequest, ClientBlockListResponse, PagingControls,
};
use sawtooth_sdk::messages::block::BlockHeader;
use sawtooth_sdk::messages::validator::Message_MessageType;

// Return the block number of the current chain head, in hex, as a string
pub fn block_number<T>(_params: Params, mut sender: T) -> Result<Value, Error> where T: MessageSender {
    let mut paging = PagingControls::new();
    paging.set_count(1);
    let mut request = ClientBlockListRequest::new();
    request.set_paging(paging);

    let request_bytes = match protobuf::Message::write_to_bytes(&request) {
        Ok(b) => b,
        Err(error) => {
            println!("ERROR serializing request: {:?}", error);
            return Err(Error::internal_error());
        },
    };

    let correlation_id = match uuid::Uuid::new(uuid::UuidVersion::Random) {
        Some(cid) => cid.to_string(),
        None => {
            println!("Error generating UUID");
            return Err(Error::internal_error());
        },
    };

    let mut future = match sender.send(Message_MessageType::CLIENT_BLOCK_LIST_REQUEST,
                                       &correlation_id, &request_bytes) {
        Ok(f) => f,
        Err(error) => {
            println!("Error unwrapping future: {:?}", error);
            return Err(Error::internal_error());
        },
    };

    let message = match future.get() {
        Ok(m) => m,
        Err(error) => {
            println!("Error getting future: {:?}", error);
            return Err(Error::internal_error());
        },
    };

    let response: ClientBlockListResponse = match protobuf::parse_from_bytes(&message.content) {
        Ok(r) => r,
        Err(error) => {
            println!("Error parsing response: {:?}", error);
            return Err(Error::internal_error());
        },
    };

    let block = &response.blocks[0];
    let block_header: BlockHeader = match protobuf::parse_from_bytes(&block.header) {
        Ok(r) => r,
        Err(error) => {
            println!("Error parsing block header: {:?}", error);
            return Err(Error::internal_error());
        }
    };

    Ok(Value::String(format!("{:#x}", block_header.block_num).into()))
}

pub fn get_block_by_hash<T>(_params: Params, mut _sender: T) -> Result<Value, Error> where T: MessageSender {
    Err(error::not_implemented())
}
pub fn get_block_by_number<T>(_params: Params, mut _sender: T) -> Result<Value, Error> where T: MessageSender {
    Err(error::not_implemented())
}
