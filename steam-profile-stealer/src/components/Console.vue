<script setup lang="ts">
const props = defineProps<{ messages: { message: string; key: string; }[] }>();
</script>

<template>
  <v-container>
    <p v-for="message in messages" :key="message.key">
      {{ message.message }}
    </p>
  </v-container>
</template>

<style scoped>
v-container {
  background: black;
  color: white;
  overflow: scroll;
  height: 500px;
}
</style>