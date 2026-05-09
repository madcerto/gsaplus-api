pipeline {
    agent { docker { image 'rust:1.85-slim' } }
    stages {
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
    }
}
