<script setup lang="ts">
import { ref, watch } from "vue";
import type { Ref } from "vue";
import { useRoute } from "vue-router";
import type { ItemWithDetails } from "./models";
import { fetchItemWithDetails } from "./api";

interface Response {
  item: ItemWithDetails,
}

const item: Ref<ItemWithDetails | null> = ref(null);

const route = useRoute()
type Parameter = typeof route.params.id

watch(() => route.params.id, fetchItem, { immediate: true })
async function fetchItem(id: Parameter) {
  if (id == null || Array.isArray(id)) return

  const responseJson: Response = await fetchItemWithDetails(id)
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
      <RouterLink :to="{ name: 'itemsOfCategory', params: { id: category.id } }">
        {{ category.name }}
      </RouterLink>
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
      <RouterLink :to="{ name: 'itemsOfCategory', params: { id: mc.id } }">
        ({{ mc.name }})
      </RouterLink>
    </li>
  </ul>
</template>

<style scoped></style>
