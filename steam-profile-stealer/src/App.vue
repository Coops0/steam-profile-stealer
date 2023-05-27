<script setup lang="ts">
import { useCookieStore } from "@/stores/cookie";
import CookieInput from "@/components/CookieInput.vue";
import { useProfileStore } from "@/stores/profile";
import Console from "@/components/Console.vue";
import { ref } from "vue";
import BothProfiles from "@/components/profiles/BothProfiles.vue";
import { useLoadingStore } from "@/stores/loading";
import { useWebsocketStore } from "@/stores/websocket";

const cookieStore = useCookieStore();
const profileStore = useProfileStore();
const loadingStore = useLoadingStore();
const websocketStore = useWebsocketStore();

function saveCookie() {
  websocketStore.send({tag: 'cookie', fields: {cookie: cookieStore.cookie!}})
  loadingStore.loading = true;
}

let targetProfile = ref('');

function refreshProfile() {
  loadingStore.loading = true;

  websocketStore.send({tag: 'refresh_profile'})
}

function fetchProfile() {
  const url = targetProfile.value;
  if (!url) return;

  loadingStore.loading = true;
  websocketStore.send({tag: 'fetch_profile', fields: {url}})
}

function stealProfile() {
  const target = profileStore.targetProfile;
  if (!target) return;

  loadingStore.loading = true;
  websocketStore.send({tag: 'steal_profile', fields: {name: target.name, image_url: target.image_url}})
}
</script>


<template>
  <v-app>
    <v-main>
      <CookieInput v-if="!cookieStore.cookie || !profileStore.selfProfile"
                   @save-cookie="saveCookie"/>

      <v-container v-else>
        <v-progress-circular indeterminate v-if="loadingStore.loading"></v-progress-circular>
        <v-row justify="center">
      <BothProfiles @refresh-profile="refreshProfile" @steal-profile="stealProfile"></BothProfiles>
        </v-row>

        <v-row justify="center" align="center">
          <v-col cols="3">
            <v-text-field
                label="Target Profile"
                v-model="targetProfile"
                :disabled="loadingStore.loading"
                @keydown.enter="fetchProfile"
            ></v-text-field>
          </v-col>
          <v-col cols="2">
            <v-btn :disabled="loadingStore.loading || !targetProfile" @click="fetchProfile">Fetch Target Profile</v-btn>
          </v-col>
        </v-row>

        <v-row justify="center" align="center">
          <v-col cols="5">
            <Console></Console>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>
