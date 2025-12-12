<script setup lang="ts">
import { ref, watch } from "vue";
import type { Ref } from "vue";
import { useRoute } from "vue-router";
import type { Category, Item } from "./models";

interface Response {
  category: Category,
  items: Item[],
}

const category: Ref<Category | null> = ref(null);
const items: Ref<Item[]>= ref([]);

const route = useRoute()
type Parameter = typeof route.params.id

watch(() => route.params.id, fetchItems, { immediate: true })
async function fetchItems(id: Parameter) {
  if (id == null || Array.isArray(id)) return

  const response = await fetch(`http://localhost:3000/categories/${id}`)
  const responseJson: Response = await response.json()
  category.value = responseJson.category
  items.value = responseJson.items
}

</script>

<template>
  <h1>Category: {{ category?.name }}</h1>
  <ul>
    <li v-for="item in items" :key="item.id">
      <RouterLink :to="{ name: 'itemDetails', params: { id: item.id } }">{{ item.name }}</RouterLink>
    </li>
  </ul>
</template>

<style scoped></style>
