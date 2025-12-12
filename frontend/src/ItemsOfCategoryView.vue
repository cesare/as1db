<script setup lang="ts">
import { ref, watch } from "vue";
import type { Ref } from "vue";
import { useRoute } from "vue-router";
import type { Category, Item } from "./models";
import { fetchItemsOfCategory } from "./api";

const category: Ref<Category | null> = ref(null);
const items: Ref<Item[]>= ref([]);

const route = useRoute()
type Parameter = typeof route.params.id

watch(() => route.params.id, fetchItems, { immediate: true })
async function fetchItems(id: Parameter) {
  if (id == null || Array.isArray(id)) return

  const responseJson = await fetchItemsOfCategory(id)
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
