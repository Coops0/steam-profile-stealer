<script setup lang="ts">
import ProfileComponent from "@/components/profiles/ProfileComponent.vue";
import { useProfileStore } from "@/stores/profile";
import { useLoadingStore } from "@/stores/loading";

const profileStore = useProfileStore();
const loadingStore = useLoadingStore();

defineEmits<{
  (event: 'refresh-profile'): void
  (event: 'steal-profile'): void
}>();

</script>

<template>
  <v-col cols="3">
    <ProfileComponent v-bind="profileStore.selfProfile" class="pa-2 ma-2">
      <v-btn
          :disabled="loadingStore.loading"
          @click="$emit('refresh-profile')"
          class="pa-2 ma-2"
          variant="outlined"
          color="teal-accent-4"
      >
        Refresh Profile
      </v-btn>
    </ProfileComponent>
  </v-col>

  <v-col cols="3" v-if="profileStore.targetProfile">
    <ProfileComponent v-bind="profileStore.targetProfile">
      <v-btn
          :disabled="loadingStore.loading"
          @click="$emit('steal-profile')"
          class="pa-2 ma-2"
          color="red"
          variant="outlined"
      >Steal Profile
      </v-btn>
    </ProfileComponent>
  </v-col>
</template>