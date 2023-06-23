const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import("./components/Landing.vue"),
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import("./components/Settings.vue"),
  },
  {
    path: '/add/server/',
    name: 'AddServer',
    component: () => import("./components/AddServer.vue"),
  },
  {
    path: '/server/:id',
    component: () => import("./components/server/[id].vue"),
  },
];
import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;