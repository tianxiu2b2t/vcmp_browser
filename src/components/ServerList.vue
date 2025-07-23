<template>
  <div class="server-list-container">
    <h2>Vice City Multiplayer Server Browser</h2>
    <table class="server-table">
      <thead>
        <tr>
          <th>Hostname</th>
          <th>Players</th>
          <th>Version</th>
          <th>Gamemode</th>
          <th>Server Info</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(server, index) in servers"
          :key="index"
          @click="selectServer(server)"
          :class="{ selected: selectedServer && selectedServer.servername === server.servername }"
        >
          <td>{{ server.servername }}</td>
          <td>{{ server.info_players }}/{{ server.info_maxplayers }}</td>
          <td>{{ server.version }}</td>
          <td>{{ server.gamemode }}</td>
          <td>
            <div v-if="selectedServer && selectedServer.servername === server.servername">
              <p>Server Name: {{ server.servername }}</p>
              <p>Server IP: {{ server.ip }}:{{ server.port }}</p>
              <p>Server Players: {{ server.info_players }}/{{ server.info_maxplayers }}</p>
              <p>Server Ping: {{ server.ping }}</p>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  name: 'ServerList',
  setup() {
    // 模拟从后端获取的服务器数据，实际应替换为真实接口请求
    const servers = ref([
      {
        servername: '[0.4] 梦幻决斗_Windows_Duell So Meu...',
        info_players: 0,
        info_maxplayers: 50,
        version: '0.4.7.1',
        gamemode: '1.14.9',
        ip: '192.168.1.100',
        port: 8888,
        ping: 14
      },
      {
        servername: '极速竞速 Extreme Racing',
        info_players: 1,
        info_maxplayers: 100,
        version: '0.4.7.1',
        gamemode: 'QQ群: 953600032',
        ip: '192.168.1.101',
        port: 8889,
        ping: 35
      },
      // 可继续添加更多模拟数据或替换为真实请求数据
    ]);
    const selectedServer = ref(null);

    // 选中服务器时的回调，展示详细信息
    const selectServer = (server) => {
      selectedServer.value = server;
    };

    return {
      servers,
      selectedServer,
      selectServer
    };
  }
};
</script>

<style scoped>
.server-list-container {
  font-family: Arial, sans-serif;
  padding: 10px;
}

.server-table {
  width: 100%;
  border-collapse: collapse;
}

.server-table th,
.server-table td {
  border: 1px solid #ccc;
  padding: 8px;
  text-align: left;
}

.server-table th {
  background-color: #f0f0f0;
}

tr:hover {
  background-color: #fafafa;
  cursor: pointer;
}

.selected {
  background-color: #e6f3ff;
}
</style>