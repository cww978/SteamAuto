import { createApp } from 'vue';
import './styles/index.css';
import App from './App.vue';

// 禁止窗口内默认浏览器右键菜单
document.addEventListener('contextmenu', (e) => {
  e.preventDefault();
}, { capture: true });

createApp(App).mount('#app');
