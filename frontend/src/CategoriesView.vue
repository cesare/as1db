<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Ref } from "vue";
import type { Category } from "./models";
import { fetchCategories } from "./api";

const categories: Ref<Category[]>= ref([]);

onMounted(async () => {
  const responseJson = await fetchCategories()
  categories.value = responseJson.categories
});

</script>

<template>
  <h1>Categories</h1>
  <ul>
    <li v-for="category in categories" :key="category.id">
      <RouterLink :to="{ name: 'itemsOfCategory', params: { id: category.id } }">
        {{ category.name }}
      </RouterLink>
    </li>
  </ul>
</template>

<style scoped></style>
