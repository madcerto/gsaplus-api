pipeline {
    agent { docker { image 'rust:1.85-slim' } }
    environment {
        POSTGRES = credentials('gsaplus_db')
    }
    stages {
        // Utilizing SCM option in Jenkins to get
        // this Jenkinsfile will already pull the
        // rest of the repository
        stage('Test') {
            steps {
                sh 'docker build -f services/gateway/Dockerfile --target=test --tag \'gsaplus-api:test\' .'
                sh 'docker run \'gsaplus-api:test\''
            }
        }
        stage('Build') {
            steps {
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
