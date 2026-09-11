// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use iggy::prelude::{
    ClientInfo as RustClientInfo, ClientInfoDetails as RustClientInfoDetails,
    ConsumerGroupInfo as RustConsumerGroupInfo,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass]
pub struct ConsumerGroupInfo {
    pub(crate) inner: RustConsumerGroupInfo,
}

impl From<&RustConsumerGroupInfo> for ConsumerGroupInfo {
    fn from(cg: &RustConsumerGroupInfo) -> Self {
        Self { inner: cg.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ConsumerGroupInfo {
    #[getter]
    pub fn stream_id(&self) -> u32 { self.inner.stream_id }
    #[getter]
    pub fn topic_id(&self) -> u32 { self.inner.topic_id }
    #[getter]
    pub fn consumer_group_id(&self) -> u32 { self.inner.group_id }
}


#[gen_stub_pyclass]
#[pyclass]
pub struct ClientInfo {
    pub(crate) inner: RustClientInfo,
}

impl From<RustClientInfo> for ClientInfo {
    fn from(c: RustClientInfo) -> Self {
        Self { inner: c }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ClientInfo {

    #[getter]
    pub fn client_id(&self) -> u32 {
        self.inner.client_id
    }


    #[getter]
    pub fn user_id(&self) -> Option<u32> {
        self.inner.user_id
    }

    #[getter]
    pub fn address(&self) -> String {
        self.inner.address.clone()
    }


    #[getter]
    pub fn transport(&self) -> String {
        self.inner.transport.clone()
    }


    #[getter]
    pub fn consumer_groups_count(&self) -> u32 {
        self.inner.consumer_groups_count
    }
}


#[gen_stub_pyclass]
#[pyclass]
pub struct ClientInfoDetails {
    pub(crate) inner: RustClientInfoDetails,
}

impl From<RustClientInfoDetails> for ClientInfoDetails {
    fn from(c: RustClientInfoDetails) -> Self {
        Self { inner: c }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ClientInfoDetails {

    #[getter]
    pub fn client_id(&self) -> u32 {
        self.inner.client_id
    }

    #[getter]
    pub fn user_id(&self) -> Option<u32> {
        self.inner.user_id
    }


    #[getter]
    pub fn address(&self) -> String {
        self.inner.address.clone()
    }


    #[getter]
    pub fn transport(&self) -> String {
        self.inner.transport.clone()
    }


    #[getter]
    pub fn consumer_groups_count(&self) -> u32 {
        self.inner.consumer_groups_count
    }


    #[getter]
    pub fn consumer_groups(&self) -> Vec<ConsumerGroupInfo> {
        self.inner
            .consumer_groups
            .iter()
            .map(ConsumerGroupInfo::from)
            .collect()
    }
}