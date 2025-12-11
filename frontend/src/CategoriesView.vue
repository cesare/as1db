<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Ref } from "vue";

interface Category {
  id: number,
  name: string,
}

interface Response {
  categories: Category[],
}

const categories: Ref<Category[]>= ref([]);

onMounted(async () => {
  const response = await fetch("http://localhost:3000/categories");
  const responseJson: Response = await response.json();
  categories.value = responseJson.categories;
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
