pipeline {
    agent { docker { image 'rust:1.85-slim' } }
    stages {
        // Utilizing SCM option in Jenkins to get
        // this Jenkinsfile will already pull the
        // rest of the repository
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
        stage('Build') {
            steps {
                sh 'docker -v'
                sh 'docker compose build'
            }
        }
        stage('Deploy') {
            steps {
                sh 'docker compose up -d'
            }
        }
    }
}
