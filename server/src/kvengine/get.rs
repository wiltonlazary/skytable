/*
 * Created on Fri Aug 14 2020
 *
 * This file is a part of the source code for the Terrabase database
 * Copyright (c) 2020, Sayan Nandan <ohsayan at outlook dot com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

//! # `GET` queries
//! This module provides functions to work with `GET` queries

use crate::coredb::CoreDB;
use crate::protocol::{responses, ActionGroup, Connection};
use crate::resp::{BytesWrapper, GroupBegin};
use libtdb::terrapipe::RespCodes;
use libtdb::TResult;

/// Run a `GET` query
pub async fn get(handle: &CoreDB, con: &mut Connection, act: ActionGroup) -> TResult<()> {
    let howmany = act.howmany();
    if howmany == 0 {
        return con.write_response(responses::ACTION_ERR.to_owned()).await;
    }
    // Write #<m>\n#<n>\n&<howmany>\n to the stream
    con.write_response(GroupBegin(howmany)).await?;
    let mut keys = act.into_iter();
    let handle = handle.acquire_read(); // Get a read lock
    while let Some(key) = keys.next() {
        if let Some(value) = handle.get(&key) {
            // Good, we got the value, write it off to the stream
            con.write_response(BytesWrapper(value.get_blob().clone()))
                .await?;
        } else {
            // Ah, couldn't find that key
            con.write_response(RespCodes::NotFound).await?;
        }
    }
    Ok(())
}