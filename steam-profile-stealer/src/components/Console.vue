<script setup lang="ts">
defineProps<{ messages: { message: string; key: string; error: boolean; }[] }>();
</script>

<template>
  <p v-for="message in messages" :key="message.key" :class="{'text-red': message.error}">
    {{ message.message }}
  </p>
</template>

<style scoped>
/*noinspection CssOverwrittenProperties*/
p {
  font-family: 'Roboto', sans-serif;
  font-family: 'Roboto Mono', monospace;
}
</style>