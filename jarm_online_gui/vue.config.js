module.exports = {
    devServer: {
        watchFiles: ["src/*"],
    },
    chainWebpack: config => {
      config.plugins.delete('preload')
      config.plugins.delete('prefetch')
    },
}