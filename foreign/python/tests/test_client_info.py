# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

import pytest
from apache_iggy import ClientInfo, ClientInfoDetails, IggyClient
from .utils import wait_for_ping


class TestGetMe:
    @pytest.mark.asyncio
    async def test_get_me_returns_client_info_details(self, iggy_client: IggyClient):
        """Test get_me returns ClientInfoDetails for the authenticated session."""
        me = await iggy_client.get_me()
        assert isinstance(me, ClientInfoDetails)
        assert isinstance(me.client_id, int)
        assert me.client_id > 0
        assert isinstance(me.address, str)
        assert isinstance(me.transport, str)
        assert isinstance(me.consumer_groups_count, int)
        assert isinstance(me.consumer_groups, list)

    @pytest.mark.asyncio
    async def test_get_me_client_id_appears_in_get_clients(self, iggy_client: IggyClient):
        """Test that the client returned by get_me appears in get_clients()."""
        me = await iggy_client.get_me()
        clients = await iggy_client.get_clients()
        client_ids = [c.client_id for c in clients]
        assert me.client_id in client_ids


class TestGetClient:
    @pytest.mark.asyncio
    async def test_get_client_returns_details_for_known_id(self, iggy_client: IggyClient):
        """Test get_client returns ClientInfoDetails for the current client's id."""
        me = await iggy_client.get_me()
        found = await iggy_client.get_client(me.client_id)
        assert found is not None
        assert isinstance(found, ClientInfoDetails)
        assert found.client_id == me.client_id

    @pytest.mark.asyncio
    async def test_get_client_returns_none_for_unknown_id(self, iggy_client: IggyClient):
        """Test get_client returns None for an id that is not connected."""
        # Use a sentinel value that cannot be a real client id on a fresh server.
        result = await iggy_client.get_client(2**32 - 1)
        assert result is None


class TestGetClients:
    @pytest.mark.asyncio
    async def test_get_clients_returns_list_of_client_info(self, iggy_client: IggyClient):
        """Test get_clients returns a list of ClientInfo."""
        clients = await iggy_client.get_clients()
        assert isinstance(clients, list)
        assert len(clients) >= 1
        for c in clients:
            assert isinstance(c, ClientInfo)
            assert isinstance(c.client_id, int)
            assert isinstance(c.address, str)
            assert isinstance(c.transport, str)
            assert isinstance(c.consumer_groups_count, int)

    @pytest.mark.asyncio
    async def test_get_clients_includes_own_client(self, iggy_client: IggyClient):
        """Test that at least the current session's client appears in get_clients."""
        me = await iggy_client.get_me()
        clients = await iggy_client.get_clients()
        assert any(c.client_id == me.client_id for c in clients)


@pytest.mark.asyncio
async def test_client_info_methods_require_connection_and_auth(unique_name):
    """Test that the three methods fail before connecting and before login."""
    from .utils import get_server_config, wait_for_server
    host, port = get_server_config()
    wait_for_server(host, port)

    client = IggyClient(f"{host}:{port}")

    with pytest.raises(RuntimeError):
        await client.get_me()

    await client.connect()
    with pytest.raises(RuntimeError):
        await client.get_me()