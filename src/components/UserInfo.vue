<script setup>
import { ref, onMounted } from 'vue'
import { getUserInfo } from "../js/store.js"

const userInfo = getUserInfo();
let balance = ref(0);
</script>

<template>
    <div class="user-info">
        <div>
            Hi, {{ userInfo.name }}

        </div>
        <div>
            余额: {{ balance }}
        </div>
    </div>
</template>

<style scoped>
.user-info {
    height: 8%;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px 10px 0 10;
    border-bottom: 1px solid rgba(154, 196, 236, 0.3);
}

.user-info>div {
    display: flex;
    align-items: center;
    padding: 5px;
}

.user-info>div:first-child {
    width: 60%;
    height: 98%;
    justify-content: flex-start;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    border-right: 1px solid rgba(154, 196, 236, 0.3);
}

.user-info>div:last-child {
    width: 40%;
    height: 98%;
    justify-content: center;
}
</style>