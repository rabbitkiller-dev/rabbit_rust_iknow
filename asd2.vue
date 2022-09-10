
<template>
    <button @click="($event) => {validate();commit();}"></button>
</template>
<script>

export default {
    data() {
        return {
            userModel: {
                username: '',
            } 
        }
    },
    methods: {
        commit(user) {
            user.username = '';
        },
        clickSave() {
            this.userModel = this.commit({})
        }
    }
}
</script>

