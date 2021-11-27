import { createRouter, createWebHistory } from 'vue-router'
import Alarms from '../views/Alarms.vue'

const routes = [
  {
    path: '/',
    name: 'Alarms',
    component: Alarms
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('@/views/About.vue')
  },
  {
    path: '/history',
    name: 'History',
    component: () => import('@/views/History.vue')
  },
  {
    path: '/api',
    name: 'API',
    component: () => import('@/views/API.vue')
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
