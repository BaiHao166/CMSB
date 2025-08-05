<script setup>
import UserInfo from './UserInfo.vue';
import Menus from './Menus.vue';
</script>

<template>
    <div class="left-area">
        <UserInfo></UserInfo>
        <Menus></Menus>
    </div>
</template>

<style scoped>
.left-area {
    width: 20%;
    height: 99.5%;
    background-color: white;
    border-radius: 10px;
    padding: 4px;
}
</style>