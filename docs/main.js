/**
 * SteamAuto Official Website - Interactive Scripts
 */

// Preset games for simulator
const PRESET_GAMES = {
  '1091500': {
    name: 'Cyberpunk 2077',
    dlcCount: 18,
    manifest: '768934120938472',
    size: '68.4 MB (Manifest & Key)',
    status: '已解锁 往日之影 & 全套特典'
  },
  '2358720': {
    name: 'Black Myth: Wukong (黑神话：悟空)',
    dlcCount: 4,
    manifest: '981273918237192',
    size: '12.8 MB (Manifest & Key)',
    status: '已就绪 豪华版升级包 & 争先红葫芦'
  },
  '1245620': {
    name: 'ELDEN RING (艾尔登法环)',
    dlcCount: 6,
    manifest: '556182930192841',
    size: '45.2 MB (Manifest & Key)',
    status: '已解锁 黄金树幽影 (Shadow of the Erdtree)'
  },
  '730': {
    name: 'Counter-Strike 2',
    dlcCount: 2,
    manifest: '339102948192019',
    size: '8.4 MB (Manifest & Key)',
    status: '已部署 优先状态认证'
  }
};

document.addEventListener('DOMContentLoaded', () => {
  initNavbarScroll();
  initFaqAccordion();
  initModuleTabs();
  initSimulator();
  initCopyButtons();
  initMobileMenu();
});

/* 1. Header scroll effect */
function initNavbarScroll() {
  const header = document.querySelector('.site-header');
  if (!header) return;

  const handleScroll = () => {
    if (window.scrollY > 24) {
      header.classList.add('scrolled');
    } else {
      header.classList.remove('scrolled');
    }
  };

  window.addEventListener('scroll', handleScroll, { passive: true });
  handleScroll();
}

/* 2. FAQ Accordion */
function initFaqAccordion() {
  const faqItems = document.querySelectorAll('.faq-item');
  faqItems.forEach(item => {
    const questionBtn = item.querySelector('.faq-question');
    if (!questionBtn) return;

    questionBtn.addEventListener('click', () => {
      const isActive = item.classList.contains('active');
      // Collapse all
      faqItems.forEach(i => i.classList.remove('active'));
      // If was not active, open it
      if (!isActive) {
        item.classList.add('active');
      }
    });
  });
}

/* 3. Module Tabs */
function initModuleTabs() {
  const tabBtns = document.querySelectorAll('.tab-btn');
  const tabContents = document.querySelectorAll('.tab-content');

  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      const targetId = btn.getAttribute('data-tab');
      if (!targetId) return;

      tabBtns.forEach(b => b.classList.remove('active'));
      tabContents.forEach(c => c.classList.remove('active'));

      btn.classList.add('active');
      const targetContent = document.getElementById(targetId);
      if (targetContent) {
        targetContent.classList.add('active');
      }
    });
  });
}

/* 4. Interactive Simulator */
function initSimulator() {
  const simInput = document.getElementById('sim-appid-input');
  const simBtn = document.getElementById('sim-run-btn');
  const chips = document.querySelectorAll('.sim-tag-chip');
  const steps = [
    document.getElementById('step-1'),
    document.getElementById('step-2'),
    document.getElementById('step-3'),
    document.getElementById('step-4')
  ];
  const terminal = document.getElementById('sim-terminal');

  if (!simInput || !simBtn || !terminal) return;

  let isRunning = false;

  const logMessage = (msg, type = 'normal') => {
    const p = document.createElement('div');
    p.className = 'console-line';
    const now = new Date().toTimeString().split(' ')[0];
    
    let colorClass = 'console-prefix';
    if (type === 'success') colorClass = 'console-success';
    if (type === 'dim') colorClass = 'text-dim';

    p.innerHTML = `<span class="${colorClass}">[${now}]</span> <span>${msg}</span>`;
    terminal.appendChild(p);
    terminal.scrollTop = terminal.scrollHeight;
  };

  const runSimulation = (appid) => {
    if (isRunning) return;
    isRunning = true;
    simBtn.disabled = true;
    simBtn.innerHTML = `<span>处理中...</span>`;

    const game = PRESET_GAMES[appid] || {
      name: `Steam AppID #${appid}`,
      dlcCount: 5,
      manifest: '882910492817291',
      size: '24.6 MB',
      status: '自动检索并解锁清单'
    };

    // Reset steps
    steps.forEach(s => {
      if (s) {
        s.classList.remove('active', 'completed');
      }
    });
    terminal.innerHTML = '';

    logMessage(`⚡ 正在为 [AppID: ${appid}] 启动自动化云端入库流程...`);

    // Step 1: Connect upstream
    setTimeout(() => {
      if (steps[0]) steps[0].classList.add('active');
      logMessage(`📡 路由选择: WUDRM 高速清单源 (Ping: 42ms)...`);
    }, 400);

    // Step 2: Fetch manifest & key
    setTimeout(() => {
      if (steps[0]) {
        steps[0].classList.remove('active');
        steps[0].classList.add('completed');
      }
      if (steps[1]) steps[1].classList.add('active');
      logMessage(`🔍 发现游戏实体: "${game.name}", 包含 ${game.dlcCount} 个 DLC 分支`);
      logMessage(`📥 正在下载 Lua 脚本包与 depotcache 清单 (${game.size})...`);
    }, 1200);

    // Step 3: Parse Lua & DepotKey
    setTimeout(() => {
      if (steps[1]) {
        steps[1].classList.remove('active');
        steps[1].classList.add('completed');
      }
      if (steps[2]) steps[2].classList.add('active');
      logMessage(`🔑 成功解密 DepotKey 与 Manifest ID: ${game.manifest}`);
      logMessage(`⚙️ 正在执行 OpenSteamTool 指令: addappid(${appid})...`);
    }, 2200);

    // Step 4: Deploy & Complete
    setTimeout(() => {
      if (steps[2]) {
        steps[2].classList.remove('active');
        steps[2].classList.add('completed');
      }
      if (steps[3]) {
        steps[3].classList.add('active', 'completed');
      }
      logMessage(`✨ [SUCCESS] 文件已安全写入 Steam 目录并完成钩子注入！`, 'success');
      logMessage(`🎮 游戏 "${game.name}" 现已假入库，请在 Steam 客户端点击安装或运行。`, 'success');

      simBtn.disabled = false;
      simBtn.innerHTML = `<span>⚡ 立即模拟入库</span>`;
      isRunning = false;
      showToast(`🎮 AppID ${appid} 模拟入库成功！`);
    }, 3200);
  };

  simBtn.addEventListener('click', () => {
    const val = simInput.value.trim();
    if (!val) {
      showToast('请输入有效的 Steam AppID');
      return;
    }
    runSimulation(val);
  });

  chips.forEach(chip => {
    chip.addEventListener('click', () => {
      chips.forEach(c => c.classList.remove('active'));
      chip.classList.add('active');
      const id = chip.getAttribute('data-id');
      if (id) {
        simInput.value = id;
        runSimulation(id);
      }
    });
  });
}

/* 5. Copy Direct Links & Toast */
function initCopyButtons() {
  const copyBtns = document.querySelectorAll('.btn-copy-link');
  copyBtns.forEach(btn => {
    btn.addEventListener('click', (e) => {
      e.preventDefault();
      const url = btn.getAttribute('data-url');
      if (!url) return;

      navigator.clipboard.writeText(url).then(() => {
        showToast('已复制下载直链到剪贴板！');
      }).catch(() => {
        // Fallback
        const ta = document.createElement('textarea');
        ta.value = url;
        document.body.appendChild(ta);
        ta.select();
        document.execCommand('copy');
        document.body.removeChild(ta);
        showToast('已复制下载直链到剪贴板！');
      });
    });
  });
}

function showToast(text) {
  let toast = document.querySelector('.toast-msg');
  if (!toast) {
    toast = document.createElement('div');
    toast.className = 'toast-msg';
    document.body.appendChild(toast);
  }

  toast.innerHTML = `
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M20 6L9 17l-5-5"/>
    </svg>
    <span>${text}</span>
  `;
  toast.classList.add('show');

  setTimeout(() => {
    toast.classList.remove('show');
  }, 2600);
}

/* 6. Mobile Menu */
function initMobileMenu() {
  const btn = document.querySelector('.mobile-menu-btn');
  const nav = document.querySelector('.nav-links');
  if (!btn || !nav) return;

  btn.addEventListener('click', () => {
    const isVisible = nav.style.display === 'flex';
    if (isVisible) {
      nav.style.display = 'none';
    } else {
      nav.style.display = 'flex';
      nav.style.flexDirection = 'column';
      nav.style.position = 'absolute';
      nav.style.top = '72px';
      nav.style.left = '0';
      nav.style.width = '100%';
      nav.style.background = 'rgba(6, 9, 14, 0.98)';
      nav.style.padding = '24px';
      nav.style.borderBottom = '1px solid rgba(255, 255, 255, 0.1)';
      nav.style.gap = '20px';
    }
  });

  // Close menu when clicking nav links
  nav.querySelectorAll('a').forEach(link => {
    link.addEventListener('click', () => {
      if (window.innerWidth <= 768) {
        nav.style.display = 'none';
      }
    });
  });
}
