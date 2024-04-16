pipeline{
    agent any
    stages {
        stage('Build Maven') {
            steps{
                checkout([$class: 'GitSCM', branches: [[name: '*/main']], extensions: [], userRemoteConfigs: [[url: 'https://github.com/AlexeyNavalniyPrinted/avel']]])
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                  sh 'docker build -t alex23451234/avel:latest -f Dockerfile ./avel/'
                }
            }
        }

        stage('Deploy Docker Image') {
            steps {
                script {
                 withCredentials([string(credentialsId: 'dockerhub-pwd', variable: 'dockerhubpwd')]) {
                    sh 'docker login -u alex23451234 -p ${dockerhubpwd}'
                 }
                 sh 'docker push alex23451234/avel:latest'
                 }
            }
        }

        stage('Deploy App on k8s') {
            steps {
                script {
                    withKubeConfig([credentialsId: 'minikube-config', variable: 'minikube-config']) {
                        sshagent([''])
                        sh 'python auto.py'
                    }
                }
            }
        }
    }
}

// use sshagent(['ssh-agent-secret-name-in-jenkins']) {} if remote k8s cluster