use std::{collections::BTreeMap, sync::Arc};

use crate::{prelude::W, utils::macros::map}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{thing, Array, Object, Value},
    Detastore, Response, Session
};

#[derive(Debug, Serialize, Deserialize)]