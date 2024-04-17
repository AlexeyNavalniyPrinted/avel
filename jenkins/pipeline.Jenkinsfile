pipeline {
    agent {
        kubernetes {
            label 'docker-build-pod'
            yaml
            """
            apiVersion: v1
            kind: Pod
            spec:
             containers:
             - name: docker
                image: docker:dind
                command: ['sleep']
                args: ['infinity']
                volumeMounts:
                - mountPath: /var/run/docker.sock
                  name: docker-sock
             volumes:
             - name: docker-sock
                hostPath:
                  path: /var/run/docker.sock
            """
        }
    }
    stages {
        stage('Pull git repository') {
            steps {
                checkout([
                    $class: 'GitSCM',
                    branches: [[name: '*/main']],
                    extensions: [],
                    userRemoteConfigs: [[url: 'https://github.com/AlexeyNavalniyPrinted/avel']]
                ])
            }
        }

        stage('Static code analysis') {
            steps {
                sh 'echo not implemented'
            }
        }

        stage('Build Image') {
            steps {
                container('docker') {
                    sh 'docker build -t alex23451234/avel:latest .'
                }
            }
        }

        stage('Push Image') {
            steps {
                container('docker') {
                    withCredentials([usernamePassword(credentialsId: 'dockerhub-credentials', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
                        sh 'docker login --user $USERNAME --password-stdin $PASSWORD'
                        sh 'docker push alex23451234/avel:latest'
                    }
                }
            }
        }
    }
}