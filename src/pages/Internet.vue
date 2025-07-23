<template>
	<div class="internet-page">
		<!-- 页面标题和操作区 -->
		<div class="page-header">
			<h1>互联网服务器列表</h1>
			<button 
				@click="fetchInternet" 
				:disabled="isLoading"
				class="refresh-btn"
			>
				{{ isLoading ? '加载中...' : '刷新列表' }}
			</button>
		</div>

		<!-- 加载状态 -->
		<div v-if="isLoading && servers.length === 0" class="loading">
			正在获取服务器列表...
		</div>

		<!-- 错误状态 -->
		<div v-if="errorMessage" class="error-message">
			{{ errorMessage }}
		</div>

		<!-- 服务器列表 -->
		<table v-else-if="servers.length > 0" class="servers-table">
			<thead>
				<tr>
					<th>服务器名称</th>
					<th>玩家数量</th>
					<th>版本</th>
					<th>游戏模式</th>
					<th>延迟</th>
				</tr>
			</thead>
			<tbody>
				<tr 
					v-for="server in servers" 
					:key="`${server.ip}:${server.port}`"
					@click="selectServer(server)"
					:class="{ 'selected': selectedServer === server }"
				>
					<td>{{ server.servername }}</td>
					<td>{{ server.info_players }}/{{ server.info_maxplayers }}</td>
					<td>{{ server.version }}</td>
					<td>{{ server.gamemode }}</td>
					<td :class="getPingClass(server.ping)">
						{{ server.ping }}ms
					</td>
				</tr>
			</tbody>
		</table>

		<!-- 空状态 -->
		<div v-else class="empty-state">
			没有找到服务器，请点击刷新按钮重试
		</div>

		<!-- 选中的服务器详情 -->
		<div v-if="selectedServer" class="server-detail">
			<h3>服务器详情</h3>
			<p><strong>名称:</strong> {{ selectedServer.servername }}</p>
			<p><strong>IP地址:</strong> {{ selectedServer.ip }}:{{ selectedServer.port }}</p>
			<p><strong>玩家:</strong> {{ selectedServer.info_players }}/{{ selectedServer.info_maxplayers }}</p>
			<p><strong>游戏模式:</strong> {{ selectedServer.gamemode }}</p>
			<p><strong>版本:</strong> {{ selectedServer.version }}</p>
			<p><strong>延迟:</strong> {{ selectedServer.ping }}ms</p>
		</div>
	</div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { onMounted, onUnmounted, reactive, ref } from 'vue';

export default {
	data() {
		return {
			servers: [],          // 服务器列表数据
			selectedServer: null, // 当前选中的服务器
			isLoading: false,     // 加载状态
			errorMessage: '',     // 错误信息
			eventListener: null   // 事件监听器引用，用于清理
		};
	},
	methods: {
		// 获取延迟状态的样式类
		getPingClass(ping) {
			if (ping < 50) return 'ping-excellent';
			if (ping < 100) return 'ping-good';
			if (ping < 200) return 'ping-average';
			return 'ping-poor';
		},

		// 选中服务器
		selectServer(server) {
			this.selectedServer = server;
		},

		// 获取服务器列表
		async fetchInternet() {
			// 重置状态
			this.isLoading = true;
			this.errorMessage = '';
			this.servers = [];
			this.selectedServer = null;

			try {
				// 监听服务器数据事件
				this.eventListener = await listen("fetch_internet", (event) => {
					// 将新服务器添加到列表
					if (event.payload) {
						this.servers.push(event.payload);
					}
				});

				// 调用后端获取服务器列表
				await invoke('fetch_internet');
			} catch (error) {
				console.error('获取服务器失败:', error);
				this.errorMessage = `获取服务器失败: ${error.message || error}`;
			} finally {
				this.isLoading = false;
			}
		},

		// 跳转到关于页面
		goToAbout() {
			this.$router.push('/about');
		}
	},
	mounted() {
		// 页面加载时自动获取服务器列表
		//this.fetchInternet();
	},
	beforeUnmount() {
		//// 页面卸载时清理事件监听器
		//if (this.eventListener) {
		//	listen(this.eventListener);
		//}
	}
};
</script>

<style scoped>
.internet-page {
	padding: 20px;
	max-width: 1200px;
	margin: 0 auto;
	font-family: Arial, sans-serif;
}

.page-header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	margin-bottom: 20px;
}

.refresh-btn {
	padding: 8px 16px;
	background-color: #3b82f6;
	color: white;
	border: none;
	border-radius: 4px;
	cursor: pointer;
	transition: background-color 0.2s;
}

.refresh-btn:disabled {
	background-color: #94a3b8;
	cursor: not-allowed;
}

.refresh-btn:hover:not(:disabled) {
	background-color: #2563eb;
}

.servers-table {
	width: 100%;
	border-collapse: collapse;
	border: 1px solid #e2e8f0;
}

.servers-table th,
.servers-table td {
	padding: 12px;
	text-align: left;
	border-bottom: 1px solid #e2e8f0;
}

.servers-table th {
	background-color: #f1f5f9;
	font-weight: 600;
}

.servers-table tr:hover {
	background-color: #f8fafc;
}

.servers-table tr.selected {
	background-color: #e0f2fe;
}

.loading, .empty-state {
	padding: 40px;
	text-align: center;
	color: #64748b;
}

.error-message {
	padding: 16px;
	background-color: #fee2e2;
	color: #dc2626;
	border-radius: 4px;
	margin-bottom: 20px;
}

.server-detail {
	margin-top: 20px;
	padding: 20px;
	background-color: #f8fafc;
	border-radius: 4px;
	border: 1px solid #e2e8f0;
}

.server-detail h3 {
	margin-top: 0;
	color: #1e293b;
}

/* 延迟状态样式 */
.ping-excellent {
	color: #10b981;
}

.ping-good {
	color: #3b82f6;
}

.ping-average {
	color: #f59e0b;
}

.ping-poor {
	color: #ef4444;
}
</style>
 