<script setup lang="ts">
import { ref, watch } from "vue";
import type { Ref } from "vue";
import { useRoute } from "vue-router";

interface Class {
  id: number,
  name: string,
}

interface Category {
  id: number,
  name: string,
}

interface Item {
  id: number,
  name: string,
}

interface ItemWithDetails {
  id: number,
  name: string,
  class: Class,
  categories: Category[],
  materialItems: Item[],
  materialCategories: Category[],
}

interface Response {
  item: ItemWithDetails,
}

const item: Ref<ItemWithDetails | null> = ref(null);

const route = useRoute()
type Parameter = typeof route.params.id

watch(() => route.params.id, fetchItems, { immediate: true })
async function fetchItems(id: Parameter) {
  if (id == null || Array.isArray(id)) return

  const response = await fetch(`http://localhost:3000/items/${id}`);
  const responseJson: Response = await response.json();
  item.value = responseJson.item
}

</script>

<template>
  <h1>Item: {{ item?.name }}</h1>

  <h2>Class</h2>
  <ul>
    <li>
      <RouterLink :to="{ name: 'itemsOfClass', params: { id: item?.class.id } }">
        {{ item?.class.name }}
      </RouterLink>
    </li>
  </ul>

  <h2>Categories</h2>
  <ul>
    <li v-for="category in item?.categories" :key="category.id">
      {{ category.name }}
    </li>
  </ul>

  <h2>Materials</h2>
  <ul>
    <li v-for="mi in item?.materialItems" :key="mi.id">
      <RouterLink :to="{ name: 'itemDetails', params: { id: mi.id } }">
        {{ mi.name }}
      </RouterLink>
    </li>

    <li v-for="mc in item?.materialCategories" :key="mc.id">
      ({{ mc.name }})
    </li>
  </ul>
</template>

<style scoped></style>
