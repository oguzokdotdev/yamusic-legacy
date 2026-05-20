const { invoke } = window.__TAURI__.core;

const token = await invoke('get_token');
if (!token) {
  window.location.replace('auth.html');
}
// Навигация по страницам
const navBtns = document.querySelectorAll('.nav-btn[data-page]');
const pages = document.querySelectorAll('.page');

navBtns.forEach(btn => {
  btn.addEventListener('click', () => {
    navBtns.forEach(b => b.classList.remove('active'));
    pages.forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    document.getElementById(`page-${btn.dataset.page}`).classList.add('active');
  });
});

// Play/pause
let playing = false;
const playBtn = document.getElementById('play-btn');
const playIcon = `<svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>`;
const pauseIcon = `<svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>`;

playBtn.addEventListener('click', () => {
  playing = !playing;
  playBtn.innerHTML = playing ? pauseIcon : playIcon;
});

// Like toggle
const likeBtn = document.querySelector('.like-btn');
likeBtn.addEventListener('click', () => {
  likeBtn.classList.toggle('active');
  const path = likeBtn.querySelector('path');
  if (likeBtn.classList.contains('active')) {
    path.setAttribute('fill', 'currentColor');
  } else {
    path.setAttribute('fill', 'none');
  }
});

// Progress bar click
const progressBar = document.querySelector('.progress-bar');
progressBar.addEventListener('click', (e) => {
  const rect = progressBar.getBoundingClientRect();
  const pct = (e.clientX - rect.left) / rect.width * 100;
  document.querySelector('.progress-fill').style.width = `${pct}%`;
});

// Volume bar click
const volumeBar = document.querySelector('.volume-bar');
volumeBar.addEventListener('click', (e) => {
  const rect = volumeBar.getBoundingClientRect();
  const pct = (e.clientX - rect.left) / rect.width * 100;
  document.querySelector('.volume-fill').style.width = `${Math.max(0, Math.min(100, pct))}%`;
});

// Сворачивание сайдбара
const sidebar = document.querySelector('.sidebar');
const sidebarToggle = document.getElementById('sidebar-toggle');

sidebarToggle.addEventListener('click', () => {
  sidebar.classList.toggle('collapsed');
});