pipeline{
    environment {
        dockerimagename = "alex23451234/avel"
        dockerImage = ""
    }
    agent any
    stages {
        stage( 'Pull git repository' ) {
            steps{
                checkout([$class: 'GitSCM', branches: [[name: '*/main']], extensions: [], userRemoteConfigs: [[url: 'https://github.com/AlexeyNavalniyPrinted/avel']]])
            }
        }

        stage( 'Static code analysis' ) {
            steps{
                sh 'echo not implemented'
            }
        }

        stage( 'Build Docker Image' ) {
            steps {
                script {
                    dockerImage = docker.build dockerimagename
                }
            }
        }

        stage( 'Push Docker Image' ) {
            steps {
                script {
                    withCredentials([string(credentialsId: 'dockerhub-credentials', variable: 'dockerhub-credentials')]) {
                        docker.withRegistry('https://registry.hub.docker.com', dockerhub-credentials) {
                            dockerImage.push("latest")
                        }
                    }
                }
            }
        }
    }
}