<template>
	<header>
		<!-- Left -->
		<div>
			<button @click="toggleSidebar">Menu</button>
			<button>Favourite</button>
			<button>Internet</button>
			<button>History</button>
		</div>
		<!-- Right -->
		<div>
			<button>Dark</button>
		</div>
	</header>
	
  	<div class="body">
		<div class="side" :class="{ hidden: isSidebarHidden }">
			<div>a</div>
		</div>
		<main :class="{ 'sidebar-hidden': isSidebarHidden }"> <!-- 顶部留出导航栏高度的空间 -->
  	  		<RouterView />
  		</main>
	</div>
</template>

<script>
export default {
  data() {
    return {
      isSidebarHidden: false
    }
  },
  methods: {
    toggleSidebar() {
      this.isSidebarHidden = !this.isSidebarHidden;
    }
  },
  mounted() {
    // 在小屏幕上默认隐藏侧边栏
    if (window.innerWidth <= 800) {
      this.isSidebarHidden = true;
    }
    
    // 监听窗口大小变化
    window.addEventListener('resize', () => {
      if (window.innerWidth > 800) {
        this.isSidebarHidden = false;
      } else {
        this.isSidebarHidden = true;
      }
    });
  }
}
</script>

<style>
:root {
	--text-color: #111;
	--text-light-color: #eee;
	--theme-color: #8edaf3;
	--sidebar-width: 200px;
}

/* 滚动条样式 */
::-webkit-scrollbar, html ::-webkit-scrollbar {
    width: 5px;
    height: 5px;
    border-radius: 10px;
}
::-webkit-scrollbar-thumb, html ::-webkit-scrollbar-thumb {
    box-shadow: inset 0 0 6px #0000;
    background-color: #666;
    border-radius: 10px;
}
::-webkit-scrollbar-track, html ::-webkit-scrollbar-track {
    box-shadow: inset 0 0 6px #0000;
    background-color: #afafaf;
    border-radius: 10px;
}

html, body {
  	padding: 0;
  	margin: 0;
  	background: rgb(240, 240, 240);
  	font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

/* 平滑滚动 */
html {
  	scroll-behavior: smooth;
	overflow: hidden;
	max-height: 100vh;
}
.body {
	overflow: hidden;
	margin-top: 56px;
	display: flex;
	flex-direction: row;
	height: 100vh;
	min-width: 680px;
}

header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 0 8px;
	height: 56px;
	background-color: transparent;
	backdrop-filter: blur(2px);
	box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
	position: fixed;
	top: 0;
	left: 0;
	width: 100%;
	z-index: 1000;
	transition: all 0.3s ease-in-out;
}
header:hover {
	backdrop-filter: blur(5px);
}
header button {
	background-color: transparent;
	border: none;
	outline: none;
	cursor: pointer;
	font-size: 16px;
	color: #333;
	padding: 8px 8px;
	border-radius: 4px;
	transition: all 0.3s ease-in-out;
}
header button:hover {
	background-color: var(--theme-color);
}
main {
	margin-left: var(--sidebar-width);
	overflow: auto;
	transition: margin-left 150ms cubic-bezier(0.4, 0, 0.2, 1);
	min-height: calc(100vh - 56px);
}
main.sidebar-hidden {
	margin-left: 0;
}
.side {
	width: var(--sidebar-width);
	height: 100%;
	background-color: transparent;
	backdrop-filter: blur(2px);
	position: fixed;
	transition: transform 150ms cubic-bezier(0.4, 0, 0.2, 1);
	box-shadow: 2px 0 5px rgba(0, 0, 0, 0.1);
}
.side.hidden {
	transform: translateX(-100%);
}

/* 媒体查询，适配小屏幕设备 */
@media (max-width: 800px) {
  main {
    margin-left: 0;
  }
  
  .side {
    z-index: 900; /* 确保侧边栏在主内容上方 */
    background-color: rgba(240, 240, 240, 0.9); /* 增加透明度使效果更好 */
  }
}
</style>
