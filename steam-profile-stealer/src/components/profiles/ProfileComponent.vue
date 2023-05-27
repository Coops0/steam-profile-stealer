<script setup lang="ts">
import type { Profile } from "@/stores/profile";

const profile = defineProps<Profile>();
</script>

<template>
  <v-card
      max-width="344"
  >
    <v-img
        :src="profile.image_url"
        height="200px"
    ></v-img>

    <v-card-title>
      {{ profile.name }}
    </v-card-title>

    <v-card-actions>
      <v-btn
          :href="profile.url"
          target="_blank"
          variant="text"
          color="deep-purple"
      >
        Link
      </v-btn>
      <slot></slot>
    </v-card-actions>

  </v-card>
</template>