# Influxdb-rs

InfluxDB Rust Client for Influx APIv2

[![Build Status](https://app.travis-ci.com/infosechoudini/influxdb-rs.svg?branch=main)](https://app.travis-ci.com/infosechoudini/influxdb-rs)

## Overview

This is the InfluxDB driver to be utilized with the Rust programming language. It is mainly ASYNC but will include future work to have synchronous actions as well. The driver is aimed to be used with the version 2 of the InfluxDB API. There can be backwards compatibility with the 1.x endpoints but that is not the true aim of this crate. 

## Status

- [x] HTTP Client
  - [x] Use Token Auth
- [ ] Server
  - [x] Ping
  - [x] Get Version
  - [x] Get Org ID 
  - [ ] Determine Aditional Capabilities
- [ ] Measurements
  - [x] Delete Measurements
  - [x] Add Points
  - [x] Add Measurements
  - [x] Add Fields
  - [x] Add Timestamps
  - [ ] Determine Additional Capabilities
- [ ] Bucket
  - [x] Create Bucket
  - [x] Delete Bucket
  - [x] Change Bucket
  - [ ] Determine Additional Capbilities
- [x] Rocket.rs Database Driver
- [ ] Queries
  - [x] Flux Queries
  - [ ] Determine Additional Capabilities
- [ ] Tests
  - [x] Auth Integration 
  - [ ] Auth Unit
  - [x] Write Integration
  - [ ] Write Unit
- [ ] Backwards Compatibility
  - [ ] Basic Auth 
  - [ ] 1.x Endpoint Query 

## Usage

### Use

```
[dependencies]
influxdb_rs = {path = https://github.com/infosechoudini/influxdb-rs }
```

### http

```Rust
use influxdb_rs::Client;
use url::Url;
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();
    
    let now = Utc::now();

    let point = Point::new("temporary")
        .add_field("foo", "bar")
        .add_field("integer", 11)
        .add_field("float", 22.3)
        .add_field("'boolean'", false)
        .add_timestamp(now.timestamp());

    let result = client.write_point(point, Some(Precision::Seconds), None).await;
    if result.is_err(){
        // Error!
    }

    let later = Utc::now().to_rfc3339().to_string();

    client.drop_measurement("temporary", &now.to_rfc3339(), &later).await.unwrap();
}
```

## Compatibility

InfluxDB APIv2 [API Document](https://docs.influxdata.com/influxdb/v2.0/api/)

Tested:
- [x] OSS 2.3
- [x] OSS 2.2
- [x] OSS 2.1


## Thanks

Because [**influxdbclient-rs**](https://github.com/driftluo/InfluxDBClient-rs) only support to the 1.x version. I read [**influxdb2-client**](https://github.com/influxdata/influxdb_iox/tree/main/influxdb2_client) and [**influxdb-client-go**](https://github.com/influxdata/influxdb-client-go) source, and then try to write a library for 1.0+ version for support for my own use.