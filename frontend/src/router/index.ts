import ClassesView from '@/ClassesView.vue'
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: "/", component: ClassesView },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
