pipeline {
    agent { docker { image 'rust:1.85-slim' } }
    stages {
        stage('Checkout') {
            steps {
                git url: 'https://github.com/madcerto/gsaplus-api', branch: 'main'
            }
        }
        stage('test') {
            steps {
                sh 'cargo test'
            }
        }
    }
}
