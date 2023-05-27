<script setup lang="ts">
import type { Profile } from "@/stores/profile";

const profile = defineProps<Profile>();
</script>

<template>
  <v-img :src="profile.image_url"></v-img>
  <p>{{ profile.name}}</p>
</template>