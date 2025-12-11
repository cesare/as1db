<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Ref } from "vue";

interface Class {
  id: number,
  name: string,
}

interface ClassesResponse {
  classes: Class[],
}

const classes: Ref<Class[]>= ref([]);

onMounted(async () => {
  const response = await fetch("http://localhost:3000/classes");
  const responseJson: ClassesResponse = await response.json();
  classes.value = responseJson.classes;
});

</script>

<template>
  <h1>Classes</h1>
  <ul>
    <li v-for="clazz in classes" :key="clazz.id">
      <router-link :to="{ name: 'itemsOfClass', params: { id: clazz.id } }">{{ clazz.name }}</router-link>
    </li>
  </ul>
</template>

<style scoped></style>
