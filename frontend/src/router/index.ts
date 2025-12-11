import ClassesView from '@/ClassesView.vue'
import ItemsOfClassView from '@/ItemsOfClassView.vue'
import ItemView from '@/ItemView.vue'
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: "/", component: ClassesView },
  { path: "/classes/:id", component: ItemsOfClassView, name: "itemsOfClass" },
  { path: "/items/:id", component: ItemView, name: "itemDetails" },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
