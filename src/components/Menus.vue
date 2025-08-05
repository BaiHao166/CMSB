<script setup>

function clickMenu(menu) {
    console.log(menu)
}
</script>

<template>
    <div class="side-menu">
        <ul>
            <li @click="clickMenu('list')">
                列表
            </li>
            <li @click="clickMenu('upload')">
                上传记录
            </li>
            <li @click="clickMenu('download')">
                下载记录
            </li>
            <li @click="clickMenu('setting')">
                设置
            </li>
        </ul>
    </div>
</template>

<style scoped>
.side-menu {
    width: 95%;
    height: 90%;
    margin-top: 1%;
    padding: 5px;
}

ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

ul>li {
    padding: 5px 20px;
    margin-top: 5px;
    border-radius: 15px;
}

ul>li:hover {
    background-color: rgba(154, 196, 236, 0.3);
    cursor: pointer;
}
</style>