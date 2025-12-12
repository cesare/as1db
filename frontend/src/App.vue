<script setup lang="ts"></script>

<template>
  <nav>
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/categories">Categories</RouterLink>
  </nav>
  <main>
    <RouterView />
  </main>
</template>

<style scoped></style>
